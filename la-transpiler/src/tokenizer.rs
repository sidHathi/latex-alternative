pub mod tokenizer {
    use core::num;
    use std::collections::HashSet;
    use std::rc;
    use crate::enums::{
        TokenType,
        TranscriptionMode,
    };
    use crate::models::token::{Token, self};
    use crate::models::transcription_state::TranscriptionState;
    use crate::constants::stopchars::stopchars;

    use crate::keywords::keywords::keyword_match;

    fn tokenize_buffer(
        recent_chars: &mut Vec<char>, 
        rc_start: &mut usize,
        rc_end: usize,
        state: &mut TranscriptionState, 
        token_list: &mut Vec<Token>, 
        fn_hint: bool) -> () {
        let word: String = recent_chars.clone().into_iter().collect();
        let token_type: TokenType = keyword_match(state, word.clone(), fn_hint);
        let new_token: Token = Token::new(
            token_type,
            Some(word),
            *rc_start,
            rc_end
        );
        *rc_start = rc_end + 1;
        println!("adding new token {}", new_token);
        token_list.push(new_token);
    }

    fn handle_indent_change(curr_state: &mut TranscriptionState, new_indentlevel: u8, t_index: usize) -> Option<Vec<Token>> {
        let indentation_dif: i16 = new_indentlevel as i16 - curr_state.indentation as i16;
        println!("indentation change detected");
        println!("current indentation: {}, new indentation: {}", curr_state.indentation, new_indentlevel);
        let mut new_tokens: Vec<Token> = Vec::new();
        if indentation_dif > 0 || (-1*indentation_dif) > curr_state.indentation as i16  ||  (curr_state.mode != TranscriptionMode::DEFAULT && curr_state.mode != TranscriptionMode::EQBLOCK) {
            println!("error: current mode = {}", curr_state.mode);
            return None;
        }
        let start_index = t_index - (-1*indentation_dif) as usize; 

        for i in 0..(-1*indentation_dif) as usize {
            new_tokens.push(Token::new(
                TokenType::BACKTAB,
                None,
                start_index + i,
                start_index + i
            ));
            curr_state.indentation -= 1;
        }
        return Some(new_tokens);
    }

    fn param_read(char_buffer: Vec<char>, start_index: &mut usize) -> Vec<Token> {
        let mut params: Vec<Token> = Vec::new();
        let param_string: String = char_buffer.clone().into_iter().collect();
        let raw_params: Vec<&str> = param_string.split(';').collect();
        let mut index = *start_index;
        for raw_param in raw_params {
            params.push(Token::new(
                TokenType::PARAM,
                Some(String::from(raw_param)),
                index,
                index + raw_param.len()
            ));
            index += raw_param.len();
        }
        *start_index = index + 1;
        return params;
    }

    fn mode_revert(curr_state: &mut TranscriptionState) -> () {
        if curr_state.prev_modes.len() > 0 {
            curr_state.mode = curr_state.prev_modes.pop().unwrap();
        }
    }

    pub fn tokenize(source: String) -> Vec<Token> {
        let mut token_list: Vec<Token> = Vec::new();
        let mut state: TranscriptionState = TranscriptionState {
            mode: TranscriptionMode::DEFAULT,
            prev_modes: Vec::new(),
            indentation: 0,
            paren_depth: 0,
        };
        let mut recent_chars: Vec<char> = Vec::new();
        let mut rc_start: usize = 0;
        let stopchar_set: HashSet<char> = HashSet::from(stopchars);
        let mut line_start: bool = true;
        let mut indentation: u8 = 0;
        for (i, c) in source.chars().enumerate() {
            let fn_hint = c == '(';
            if c == '\n' {
                if recent_chars.len() > 0 {
                    tokenize_buffer(&mut recent_chars, &mut rc_start, i-1, &mut state, &mut token_list, fn_hint);
                    recent_chars.clear();
                }
                line_start = true;
                indentation = 0;
                continue;
            } else if line_start && c == '\t' {
                indentation += 1;
                continue;
            } else {
                if line_start && indentation != state.indentation {
                    rc_start = i;
                    let backtabs: Option<Vec<Token>> = handle_indent_change(&mut state, indentation, i);
                    if backtabs.is_none() {
                        token_list.push(Token::new(
                            TokenType::ERROR,
                            None,
                            rc_start,
                            i,
                        ))
                    } else {
                        for token in backtabs.unwrap() {
                            token_list.push(token);
                        }
                    }
                }
                line_start = false;
            }
            match state.mode {
                TranscriptionMode::DEFAULT | TranscriptionMode::SPECIALTEXT => {
                    // should be searching for block and equation starting keywords/symbols - !!STILL NEED TO ADD EQ MONITORING!!
                    if stopchar_set.contains(&c) && recent_chars.len() > 0 {
                        tokenize_buffer(&mut recent_chars, &mut rc_start, i-1, &mut state, &mut token_list, fn_hint);
                        recent_chars.clear();
                    } else {
                        if c == '<' && !recent_chars.last().is_none() && *recent_chars.last().unwrap() != '\\' {
                            tokenize_buffer(&mut recent_chars, &mut rc_start, i - 1, &mut state, &mut token_list, fn_hint);
                            token_list.push(Token::new(
                                TokenType::EQ,
                                None,
                                rc_start, 
                                i,
                            ));
                            state.prev_modes.push(state.mode);
                            state.mode = TranscriptionMode::EQ;
                            continue;
                        } else if c =='<'  && recent_chars.last().is_none() {
                            token_list.push(Token::new(
                                TokenType::EQ,
                                None,
                                rc_start,
                                i
                            ));
                            rc_start = i + 1;
                            state.prev_modes.push(state.mode);
                            state.mode = TranscriptionMode::EQ;
                            continue;
                        }
                        recent_chars.push(c);
                    }

                    if state.mode == TranscriptionMode::SPECIALTEXT {
                        if c == ')' && (recent_chars.len() == 0 || *recent_chars.last().clone().unwrap() != '\\') {
                            mode_revert(&mut state);
                            continue;
                        }
                    }
                },
                TranscriptionMode::EQ | TranscriptionMode::EQBLOCK => {
                    if c == '"' && recent_chars.len() == 0 {
                        state.prev_modes.push(state.mode.clone());
                        state.mode = TranscriptionMode::DEFAULT;
                        rc_start = i + 1;
                        continue;
                    }

                    if state.mode == TranscriptionMode::EQ && c == '>' && *recent_chars.last().unwrap() == '/' {
                        recent_chars.pop();
                        tokenize_buffer(&mut recent_chars, &mut rc_start, i-2, &mut state, &mut token_list, fn_hint);
                        token_list.push(Token::new(
                            TokenType::EQEND,
                            None,
                            rc_start,
                            i,
                        ));
                        mode_revert(&mut state);
                        recent_chars.clear();
                        continue;
                    }

                    if (c == '"' || stopchar_set.contains(&c)) && recent_chars.len() > 0 {
                        tokenize_buffer(&mut recent_chars, &mut rc_start, i-1, &mut state, &mut token_list, fn_hint);
                        recent_chars.clear();
                    } else {
                        recent_chars.push(c);
                    }
                    if c == '"' {
                        state.prev_modes.push(state.mode.clone());
                        state.mode = TranscriptionMode::DEFAULT;
                    }
                },
                TranscriptionMode::PARAMREAD => {
                    if c.is_whitespace() {
                        continue;
                    }
                    if c == ')' && recent_chars.len() > 0 {
                        // NEED TO MONITOR FOR EQ BLOCK
                        for param in param_read(recent_chars.clone(), &mut rc_start) {
                            if !param.val.is_none() && param.clone().val.unwrap().eq("eq") && !token_list.last().is_none() && token_list.last().unwrap().token_type == TokenType::BLOCK {
                                println!("entering eq block mode");
                                state.prev_modes.push(state.mode);
                                state.mode = TranscriptionMode::EQBLOCK;
                                token_list.last_mut().unwrap().token_type = TokenType::EQBLOCK;
                            }
                            token_list.push(param);
                        }
                        recent_chars.clear();
                        if state.mode == TranscriptionMode::EQBLOCK {
                            continue;
                        }
                        let return_mode: Option<TranscriptionMode> = state.prev_modes.pop();
                        if !return_mode.is_none() {
                            state.mode = return_mode.unwrap();
                        }
                    } else {
                        recent_chars.push(c);
                    }
                },
                _ => ()
            }
            if i == source.len() - 1 {
                if recent_chars.len() > 0 {
                    tokenize_buffer(&mut recent_chars, &mut rc_start, i, &mut state, &mut token_list, fn_hint);
                    recent_chars.clear();
                }
            }
        }
        return token_list;
    }
}
