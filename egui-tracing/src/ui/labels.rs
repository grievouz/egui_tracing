use std::borrow::Cow;

/// Contains text labels used in the UI.
///
/// Built-in languages are available via [`Labels::english`] and
/// [`Labels::portuguese`]. For other languages, start from
/// [`Labels::default()`] (English) and override the fields you need.
/// Uses `Cow<'static, str>` so the built-ins are zero-alloc.
///
/// ```
/// let mut labels = egui_tracing::Labels::default();
/// labels.time = "Zeit".into();
/// labels.level = "Stufe".into();
/// ```
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Labels {
    // Table header
    pub time: Cow<'static, str>,
    pub level: Cow<'static, str>,
    pub target: Cow<'static, str>,
    pub message: Cow<'static, str>,

    // Buttons
    pub clear: Cow<'static, str>,
    pub to_bottom: Cow<'static, str>,
    pub close: Cow<'static, str>,

    // Detail panel
    pub event_details: Cow<'static, str>,
    pub message_too_long: Cow<'static, str>,

    // Level filter popup
    pub level_filter: Cow<'static, str>,

    // Target filter popup
    pub target_filter: Cow<'static, str>,
    pub add: Cow<'static, str>,
    pub delete: Cow<'static, str>,
    pub target_placeholder: Cow<'static, str>,
}

impl Labels {
    /// English labels (same as [`Labels::default`]).
    pub fn english() -> Self {
        Self {
            time: "Time".into(),
            level: "Level".into(),
            target: "Target".into(),
            message: "Message".into(),
            clear: "Clear".into(),
            to_bottom: "To Bottom".into(),
            close: "Close".into(),
            event_details: "Event Details".into(),
            message_too_long: "(message too long)".into(),
            level_filter: "Level Filter".into(),
            target_filter: "Target Filter".into(),
            add: "Add".into(),
            delete: "Delete".into(),
            target_placeholder: "example: eframe::*".into(),
        }
    }

    /// Portuguese labels.
    pub fn portuguese() -> Self {
        Self {
            time: "Tempo".into(),
            level: "Nível".into(),
            target: "Alvo".into(),
            message: "Mensagem".into(),
            clear: "Limpar".into(),
            to_bottom: "Até ao Fundo".into(),
            close: "Fechar".into(),
            event_details: "Detalhes do Evento".into(),
            message_too_long: "(mensagem muito longa)".into(),
            level_filter: "Filtro de Nível".into(),
            target_filter: "Filtro de Alvo".into(),
            add: "Adicionar".into(),
            delete: "Excluir".into(),
            target_placeholder: "exemplo: eframe::*".into(),
        }
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self::english()
    }
}
