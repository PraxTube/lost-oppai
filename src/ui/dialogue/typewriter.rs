use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::npc::NpcDialogue;
use crate::player::chat::PlayerStoppedChat;
use crate::{GameAssets, GameState};

use super::audio::PlayBlipEvent;
use super::option_selection::OptionSelection;
use super::spawn::{create_dialogue_text, DialogueContent, DialogueContinueNode};
use super::DialogueViewSystemSet;

#[derive(Event)]
pub struct TypewriterFinished;
#[derive(Event)]
pub struct WriteDialogueText;

#[derive(Resource)]
pub struct Typewriter {
    pub character_name: Option<String>,
    pub current_text: String,
    pub graphemes_left: Vec<String>,
    pub last_before_options: bool,
    elapsed: f32,
    start: Instant,
    last_finished: bool,
    current_speed: f32,
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            character_name: default(),
            current_text: default(),
            graphemes_left: default(),
            last_before_options: default(),
            elapsed: default(),
            start: Instant::now(),
            last_finished: default(),
            current_speed: 25.0,
        }
    }
}

impl Typewriter {
    pub fn set_completed_line(&mut self, line: &LocalizedLine) {
        *self = Self {
            character_name: line.character_name().map(|s| s.to_string()),
            current_text: line.text_without_character_name(),
            last_finished: true,
            last_before_options: line.is_last_line_before_options(),
            current_speed: self.current_speed,
            ..default()
        }
    }

    pub fn set_line(&mut self, line: &LocalizedLine) {
        *self = Self {
            character_name: line.character_name().map(|s| s.to_string()),
            current_text: String::new(),
            graphemes_left: line
                .text_without_character_name()
                .graphemes(true)
                .map(|s| s.to_string())
                .collect(),
            last_before_options: line.is_last_line_before_options(),
            current_speed: self.current_speed,
            ..default()
        };
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn is_finished(&self) -> bool {
        self.graphemes_left.is_empty() && !self.current_text.is_empty()
    }

    pub fn set_type_speed(&mut self, speed: f32) {
        self.current_speed = speed;
    }

    fn update_current_text(&mut self) -> String {
        if self.is_finished() {
            return String::new();
        }
        self.elapsed += self.start.elapsed().as_secs_f32();
        self.start = Instant::now();

        let calculated_graphemes = (self.graphemes_per_second() * self.elapsed).floor() as usize;
        let graphemes_left = self.graphemes_left.len();
        let grapheme_length_to_take = (calculated_graphemes).min(graphemes_left);

        self.elapsed -= grapheme_length_to_take as f32 / self.graphemes_per_second();
        let graphemes_to_take = self
            .graphemes_left
            .drain(..grapheme_length_to_take)
            .collect::<Vec<String>>()
            .concat();
        if graphemes_to_take.contains('.') {
            self.elapsed -= 0.2;
        } else if graphemes_to_take.contains('?') {
            self.elapsed -= 0.35;
        } else if graphemes_to_take.contains(',') {
            self.elapsed -= 0.1;
        }
        self.current_text += &graphemes_to_take;
        graphemes_to_take.to_string()
    }

    fn graphemes_per_second(&self) -> f32 {
        self.current_speed
    }

    fn finish_current_line(&mut self) {
        if self.is_finished() {
            return;
        }
        let remaining_graphemes = self.graphemes_left.drain(..);
        self.current_text.extend(remaining_graphemes);
    }
}

fn write_text(
    assets: Res<GameAssets>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut q_text: Query<&mut Text, With<DialogueContent>>,
    mut ev_write_dialogue_text: EventReader<WriteDialogueText>,
    mut ev_play_blip: EventWriter<PlayBlipEvent>,
) {
    let mut text = match q_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if !ev_write_dialogue_text.is_empty() {
        ev_write_dialogue_text.clear();
        *text = create_dialogue_text(&typewriter.current_text, "", &assets);
        return;
    }

    if typewriter.last_before_options && option_selection.is_none() {
        return;
    }
    if typewriter.is_finished() {
        return;
    }

    let added_text = typewriter.update_current_text();

    if !added_text.is_empty() && &added_text != " " {
        ev_play_blip.send(PlayBlipEvent::new(
            &typewriter.character_name.clone().unwrap_or_default(),
        ));
    }

    let rest = typewriter.graphemes_left.join("");
    *text = create_dialogue_text(&typewriter.current_text, rest, &assets);
}

fn show_continue(
    typewriter: Res<Typewriter>,
    mut q_visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
    mut ev_typewriter_finished: EventReader<TypewriterFinished>,
    mut ev_write_dialogue_text: EventReader<WriteDialogueText>,
) {
    if ev_typewriter_finished.is_empty() && ev_write_dialogue_text.is_empty() {
        return;
    }
    ev_typewriter_finished.clear();
    ev_write_dialogue_text.clear();

    let mut visibility = match q_visibility.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let vis = if typewriter.last_before_options {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };
    *visibility = vis;
}

fn send_finished_event(
    mut typewriter: ResMut<Typewriter>,
    mut ev_typewriter_finished: EventWriter<TypewriterFinished>,
) {
    if typewriter.is_finished() && !typewriter.last_finished {
        ev_typewriter_finished.send(TypewriterFinished);
        typewriter.last_finished = true;
    }
}

fn finish_stopped_dialoauge(
    mut typewriter: ResMut<Typewriter>,
    mut ev_player_stopped_chat: EventReader<PlayerStoppedChat>,
) {
    if ev_player_stopped_chat.is_empty() {
        return;
    }
    ev_player_stopped_chat.clear();

    typewriter.finish_current_line();
}

fn set_writer_speed(
    mut typewriter: ResMut<Typewriter>,
    mut ev_present_line: EventReader<PresentLineEvent>,
) {
    for ev in ev_present_line.read() {
        let name = ev
            .line
            .character_name()
            .unwrap_or_default()
            .trim_start_matches('_');
        let maybe_npc = NpcDialogue::from_str(name);
        let speed = if let Ok(npc) = maybe_npc {
            match npc {
                NpcDialogue::Jotem => 18.0,
                NpcDialogue::Eleonore => 20.0,
                NpcDialogue::Joanna => 23.0,
            }
        } else {
            match name {
                "You" => 20.0,
                _ => 15.0,
            }
        };
        typewriter.set_type_speed(speed);
    }
}

pub struct DialogueTypewriterPlugin;

impl Plugin for DialogueTypewriterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_finished_event,
                write_text,
                show_continue,
                finish_stopped_dialoauge,
                set_writer_speed,
            )
                .chain()
                .after(YarnSpinnerSystemSet)
                .in_set(DialogueViewSystemSet)
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<Typewriter>()
        .add_event::<TypewriterFinished>()
        .add_event::<WriteDialogueText>();
    }
}
