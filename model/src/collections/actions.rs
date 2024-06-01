use enum_decompose::decompose;
use scrapcore_serialization::derive::DatabaseModel;

#[derive(Debug, DatabaseModel)]
// #[model_serde(tag = "type")]
pub enum ActionOrChain {
    #[model_serde(other)]
    /// Single combat action
    Effects(ActionEffect),
    /// Chain of combat actions, each requiring the previous to complete
    #[model(no_condition)]
    Chain(CombatActionChain),
}

#[derive(Debug, DatabaseModel)]
pub struct CombatActionChain {
    pub actions: Vec<ActionOrChain>,
    pub ty: ChainType,
}

#[derive(Debug, Copy, Clone, DatabaseModel)]
pub enum ChainType {
    /// Chain always succeeds
    None,
    /// Chain only succeeds if all actions succeed
    All,
    /// Chain succeeds if any action succeeds
    Any,
    /// Same as `Any`, but chain ends early if any action fails
    Sequence,
}

/// Possible effects that an action can have
///
/// Almost everything in the game is "action" and can be canceled by other
/// actions or long-lasting effects. System operates akin to Magic The
/// Gathering stack, where events resolve from the top of the stack to the
/// bottom, and new actions can appear on top of the stack during the process
///
/// Some actions are not intended to be used by mod makers
#[decompose]
#[derive(Debug, Clone, DatabaseModel)]
#[model_serde(tag = "type")]
pub enum ActionEffect {
    DebugLog { message: String },
}
