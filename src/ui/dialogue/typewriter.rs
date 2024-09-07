use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::npc::NpcDialogue;
use crate::player::chat::PlayerStoppedChat;
use crate::ui::main_menu::{ButtonAction, MainMenuButtonPressed};
use crate::{GameAssets, GameState};

use super::audio::PlayBlipEvent;
use super::option_selection::OptionSelection;
use super::spawn::{create_dialogue_text, DialogueContent};
use super::DialogueViewSystemSet;

// The average dialogue speed.
// It's used to calculate the multiplier of the pauses caused by punctuation.
const AVERAGE_SPEED: f32 = 20.0;

#[derive(Event)]
pub struct TypewriterFinished;
/// This event triggers the typewriter to write a full line instantly.
/// It's used to set the dialuge lines whenever the player stops/starts
/// dialogue with NPCs.
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
    speed_multiplier: f32,
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
            current_speed: AVERAGE_SPEED,
            speed_multiplier: 1.0,
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
            speed_multiplier: self.speed_multiplier,
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
            speed_multiplier: self.speed_multiplier,
            ..default()
        };
    }

    pub fn reset(&mut self) {
        let speed_multiplier = self.speed_multiplier;
        *self = Self::default();
        self.speed_multiplier = speed_multiplier;
    }

    pub fn is_finished(&self) -> bool {
        self.graphemes_left.is_empty() && !self.current_text.is_empty()
    }

    fn update_current_text(&mut self) -> String {
        if self.is_finished() {
            return String::new();
        }
        self.elapsed += self.start.elapsed().as_secs_f32();
        self.start = Instant::now();

        let speed = self.current_speed * self.speed_multiplier;

        let calculated_graphemes = (speed * self.elapsed).floor() as usize;
        let graphemes_left = self.graphemes_left.len();
        let grapheme_length_to_take = (calculated_graphemes).min(graphemes_left);

        self.elapsed -= grapheme_length_to_take as f32 / speed;
        let graphemes_to_take = self
            .graphemes_left
            .drain(..grapheme_length_to_take)
            .collect::<Vec<String>>()
            .concat();

        let multiplier = AVERAGE_SPEED / speed;
        if graphemes_to_take.contains('?') {
            self.elapsed -= 0.35 * multiplier;
        } else if graphemes_to_take.contains(':') {
            self.elapsed -= 0.3 * multiplier;
        } else if graphemes_to_take.contains('-') {
            self.elapsed -= 0.25 * multiplier;
        } else if graphemes_to_take.contains('!') {
            self.elapsed -= 0.2 * multiplier;
        } else if graphemes_to_take.contains('.') {
            if let Some(index) = graphemes_to_take.chars().rev().position(|c| c == '.') {
                if index + 1 < graphemes_to_take.len()
                    || !self.graphemes_left.is_empty() && !self.graphemes_left[0].starts_with('.')
                {
                    self.elapsed -= 0.2 * multiplier;
                }
            }
        } else if graphemes_to_take.contains(',') {
            self.elapsed -= 0.1 * multiplier;
        }
        self.current_text += &graphemes_to_take;
        graphemes_to_take.to_string()
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
    if added_text.is_empty() {
        return;
    }

    if &added_text != " " {
        ev_play_blip.send(PlayBlipEvent::new(
            &typewriter.character_name.clone().unwrap_or_default(),
        ));
    }

    let rest = typewriter.graphemes_left.join("");
    *text = create_dialogue_text(&typewriter.current_text, rest, &assets);
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
                NpcDialogue::Isabelle => 20.0,
                NpcDialogue::Ionas => 16.0,
                NpcDialogue::Antonius => 19.0,
                // Should never happen!
                NpcDialogue::IonasAndAntonius => 300.0,
            }
        } else {
            match name {
                "You" => 20.0,
                _ => AVERAGE_SPEED,
            }
        };
        typewriter.current_speed = speed;
    }
}

fn update_speed_multiplier(
    mut typewriter: ResMut<Typewriter>,
    mut ev_main_menu_button_pressed: EventReader<MainMenuButtonPressed>,
) {
    for ev in ev_main_menu_button_pressed.read() {
        let speed_multiplier = match ev.0 {
            ButtonAction::Normal => 1.0,
            ButtonAction::Quick => 2.0,
            ButtonAction::Fast => 10.0,
            ButtonAction::Instant => 500.0,
            _ => continue,
        };
        typewriter.speed_multiplier = speed_multiplier;
    }
}

pub struct DialogueTypewriterPlugin;

impl Plugin for DialogueTypewriterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Typewriter>()
            .add_event::<TypewriterFinished>()
            .add_event::<WriteDialogueText>()
            .add_systems(
                Update,
                update_speed_multiplier.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                Update,
                (
                    send_finished_event,
                    write_text,
                    finish_stopped_dialoauge,
                    set_writer_speed,
                )
                    .chain()
                    .after(YarnSpinnerSystemSet)
                    .in_set(DialogueViewSystemSet)
                    .run_if(resource_exists::<GameAssets>),
            );
    }
}
