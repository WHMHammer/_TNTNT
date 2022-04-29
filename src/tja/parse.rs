#[derive(PartialEq)]
pub enum State {
    Comment,
    MetaKey,
    MetaValue,
    CommandKey,
    CommandValue,
    Measure,
}

#[derive(Clone, Copy)]
pub struct Context {
    pub measure: (u8, u8), // numerator, denominator (#MEASURE numerator,denominator)
    pub bpm: f64,
    pub flag_barline: bool,
    pub measure_notes_count: u8,
    pub offset: f64,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            measure: (4, 4),
            bpm: 120.0,
            flag_barline: true,
            measure_notes_count: 0,
            offset: 0.0,
        }
    }
}

fn second_per_note(context: &Context) -> f64 {
    60.0 / context.bpm * context.measure.0 as f64 / context.measure_notes_count as f64
}

pub fn move_events(
    measure: &mut std::collections::VecDeque<super::course::event::EventType>,
    events: &mut Vec<super::course::Event>,
    context: &mut Context,
) {
    if measure.is_empty() {
        return;
    }
    let mut offset = second_per_note(context);
    use super::course::event::*;
    while let Some(event_type) = measure.pop_front() {
        match event_type {
            Empty => {
                context.offset += offset;
            }
            Don | Ka | DON | KA | Drumroll | DRUMROLL | Balloon | End | BALLOON => {
                events.push(super::course::Event {
                    offset: context.offset,
                    event_type,
                });
                context.offset += offset;
            }
            MEASURE(numerator, denominator) => {
                context.measure = (numerator, denominator);
                offset = second_per_note(context);
            }
            BPMCHANGE(bpm) => {
                context.bpm = bpm;
                offset = second_per_note(context);
            }
            DELAY(delay) => {
                context.offset += delay;
            }
            BARLINEOFF => {
                context.flag_barline = false;
            }
            BARLINEON => {
                context.flag_barline = true;
            }
            _ => {
                events.push(super::course::Event {
                    offset: context.offset,
                    event_type,
                });
            }
        }
    }
    if context.flag_barline {
        events.push(super::course::Event {
            offset: context.offset,
            event_type: super::course::event::BARLINE,
        });
    }
    context.measure_notes_count = 0;
}
