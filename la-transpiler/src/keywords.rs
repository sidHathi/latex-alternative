pub mod keywords {
    use std::collections::HashSet;
    use crate::constants::mathkeys::{
        mathfnkeys,
        mathsymkeys
    };
    use crate::constants::syntaxkeys:: {
        stylekeys,
        logickeys
    };
    use crate::enums::{
        TranscriptionMode,
        TokenType
    };
    use crate::models::transcription_state::TranscriptionState;

    fn parse_def_key(current_state: & mut TranscriptionState, keyword: String, fn_hint: bool) -> TokenType {
        match keyword.as_str() {
            "block" => {
                if fn_hint {
                    current_state.paren_depth += 1;
                    current_state.prev_modes.push(current_state.mode.clone());
                    current_state.mode = TranscriptionMode::PARAMREAD;
                    current_state.indentation += 1;
                    return TokenType::BLOCK;
                }
                return TokenType::PLAINTEXT;
            },
            "*" => {
                current_state.prev_modes.push(current_state.mode.clone());
                current_state.mode = TranscriptionMode::ANNOREAD;
                return TokenType::ENUMITEM;
            },
            _ => TokenType::PLAINTEXT
        }
    }

    fn parse_eq_key(current_state: & mut TranscriptionState, fn_hint: bool) -> TokenType {
        if fn_hint {
            current_state.prev_modes.push(current_state.mode);
            current_state.mode = TranscriptionMode::PARAMREAD;
            return TokenType::LXFUNC;
        }
        return TokenType::LXSYMBOL;
    }

    /**
     * Determines whether a sequence of characters matches a state or command
     * keyword. Alters the transcription state based on the keyword itself.
     * 
     */
    pub fn keyword_match(current_state: &mut TranscriptionState, keyword: String, fn_hint: bool) -> TokenType {
        let mathfn_set: HashSet<& str> = HashSet::from(mathfnkeys);
        let mathsym_set: HashSet<& str> = HashSet::from(mathsymkeys);
        let logickey_set: HashSet<& str> = HashSet::from(logickeys);
        let stylekey_set: HashSet<& str> = HashSet::from(stylekeys);

        match current_state.mode {
            TranscriptionMode::DEFAULT => {
                if logickey_set.contains(keyword.as_str()) {
                    return parse_def_key(current_state, keyword, fn_hint);
                } else if stylekey_set.contains(keyword.as_str()) {
                    current_state.prev_modes.push(current_state.mode.clone());
                    current_state.mode = TranscriptionMode::SPECIALTEXT;
                    return TokenType::STYLE;
                }
                return TokenType::PLAINTEXT;
            },
            TranscriptionMode::EQ | TranscriptionMode::EQBLOCK => {
                if mathfn_set.contains(keyword.as_str()) || mathsym_set.contains(keyword.as_str()) {
                    let val_fn_hint = mathfn_set.contains(keyword.as_str()) && fn_hint;
                    return parse_eq_key(current_state, val_fn_hint);
                }
                return TokenType::PLAINTEXT;
            },
            _ => {
                return TokenType::PLAINTEXT
            }
        }
    }
}