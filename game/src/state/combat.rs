#[derive(Debug)]
pub struct CombatState {
    action: Vec<CombatAction>,
    pending_actions: Vec<CombatAction>,
}

#[enum_decompose::decompose]
#[derive(Debug, Clone)]
pub enum CombatAction {
    Start,
}
