use unicode_segmentation::UnicodeSegmentation;

use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::{GameAssets, GameState};

use super::option_selection::OptionSelection;
use super::spawn::{
    create_dialogue_text, DialogueContent, DialogueContinueNode, DialogueRoot,
    INITIAL_DIALOGUE_CONTINUE_BOTTOM,
};
use super::DialogueViewSystemSet;

#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
pub struct TypewriterFinishedEvent;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Typewriter {
    pub character_name: Option<String>,
    pub current_text: String,
    pub graphemes_left: Vec<String>,
    pub last_before_options: bool,
    elapsed: f32,
    start: Instant,
    fast_typing: bool,
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
            fast_typing: default(),
        }
    }
}

impl Typewriter {
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
            ..default()
        };
    }

    pub fn is_finished(&self) -> bool {
        self.graphemes_left.is_empty() && !self.current_text.is_empty()
    }

    pub fn fast_forward(&mut self) {
        self.fast_typing = true;
    }

    fn update_current_text(&mut self) {
        if self.is_finished() {
            return;
        }
        self.elapsed += self.start.elapsed().as_secs_f32();
        self.start = Instant::now();
        let calculated_graphemes = (self.graphemes_per_second() * self.elapsed).floor() as usize;
        let graphemes_left = self.graphemes_left.len();
        let grapheme_length_to_take = (calculated_graphemes).min(graphemes_left);
        self.elapsed -= grapheme_length_to_take as f32 / self.graphemes_per_second();
        let graphemes_to_take = self.graphemes_left.drain(..grapheme_length_to_take);
        self.current_text.extend(graphemes_to_take);
    }

    fn graphemes_per_second(&self) -> f32 {
        if self.fast_typing {
            120.0
        } else {
            40.0
        }
    }
}

fn write_text(
    assets: Res<GameAssets>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut q_text: Query<&mut Text, With<DialogueContent>>,
    mut q_root_visibility: Query<&mut Visibility, With<DialogueRoot>>,
) {
    let mut text = match q_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    if typewriter.last_before_options && option_selection.is_none() {
        *text = default();
        return;
    }
    if typewriter.is_finished() {
        return;
    }

    if !typewriter.last_before_options {
        // If this is last before options, the `OptionSelection`
        // will make the visibility inherited as soon as it's ready instead
        *q_root_visibility.single_mut() = Visibility::Inherited;
    }
    typewriter.update_current_text();

    let current_text = &typewriter.current_text;
    let rest = typewriter.graphemes_left.join("");
    *text = create_dialogue_text(current_text, rest, &assets);
}

fn show_continue(
    typewriter: Res<Typewriter>,
    mut visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>,
) {
    for _event in typewriter_finished_event.read() {
        if !typewriter.last_before_options {
            let mut visibility = visibility.single_mut();
            *visibility = Visibility::Inherited;
        }
    }
}

pub fn despawn(mut commands: Commands) {
    commands.remove_resource::<Typewriter>();
}

pub fn spawn(mut commands: Commands) {
    commands.init_resource::<Typewriter>();
}

fn bob_continue(
    time: Res<Time>,
    visibility: Query<&Visibility, With<DialogueContinueNode>>,
    mut style: Query<&mut Style, With<DialogueContinueNode>>,
) {
    let visibility = visibility.single();
    if *visibility == Visibility::Hidden {
        return;
    }
    let mut style = style.single_mut();
    let pixels =
        (time.elapsed_seconds() * 3.0).sin().powi(2) * 5.0 + INITIAL_DIALOGUE_CONTINUE_BOTTOM;
    style.bottom = Val::Px(pixels);
}

fn send_finished_event(
    mut events: EventWriter<TypewriterFinishedEvent>,
    typewriter: Res<Typewriter>,
    mut last_finished: Local<bool>,
) {
    if !typewriter.is_finished() {
        *last_finished = false;
    } else if !*last_finished {
        events.send(TypewriterFinishedEvent);
        *last_finished = true;
    }
}

pub struct DialogueTypewriterPlugin;

impl Plugin for DialogueTypewriterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_finished_event.run_if(resource_exists::<Typewriter>()),
                despawn.run_if(on_event::<DialogueCompleteEvent>()),
                spawn.run_if(on_event::<DialogueStartEvent>()),
                write_text.run_if(resource_exists::<Typewriter>()),
                show_continue.run_if(resource_exists::<Typewriter>()),
                bob_continue,
            )
                .chain()
                .after(YarnSpinnerSystemSet)
                .in_set(DialogueViewSystemSet)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<TypewriterFinishedEvent>();
    }
}
