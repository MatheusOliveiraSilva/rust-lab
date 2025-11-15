// Condições obrigatórias:
// Se start > end, deve retornar None.
// Se end > s.len(), deve retornar None.
// Caso contrário, deve retornar uma fatia da string original:
// Some(&s[start..end])
// Sem clonagem (String::from proibido).
// Sem alocação.
// Sem converter em String.
// Sem copiar bytes.

pub fn take_slice<'a>(s: &'a str, start: usize, end: usize) -> Option<&'a str> {
    if start > end || end > s.len() { return None }
    if !s.is_char_boundary(start) || !s.is_char_boundary(end) { return None }
    Some(&s[start..end])
}

// Regras da função
// Se start == end
// Não há seleção -> retorne None (ou, se quiser, Some(""), mas vamos usar None pra ficar sem ambiguidade).
// Se start > end
// Interprete isso como “o usuário arrastou o mouse de trás pra frente”.
// Você deve trocar os valores internamente:
// let (from, to) = if start <= end { (start, end) } else { (end, start) };
// Se o intervalo final (normalizado) estiver fora dos limites do buffer:
// to > buffer.len() → retorne None.
// Se qualquer das duas extremidades não estiver em char_boundary:
// mesma lógica do take_slice
// retorne None se cair no meio de um caractere UTF-8.
// Caso válido:
// retorne Some(&buffer[from..to]).
//
// Resumindo:
// Você vai normalizar o range (min/max), checar limites, checar UTF-8, e fatiar.

pub fn select_range<'a>(buffer: &'a str, start: usize, end: usize) -> Option<&'a str> {
    if start == end { return None }
    let (from, to) = if start <= end { (start, end) } else { (end, start) };

    if to > buffer.len() { return None }
    if !buffer.is_char_boundary(from) || !buffer.is_char_boundary(to) { return None}

    Some(&buffer[from..to])
}

pub fn split_once<'a>(input: &'a str, delimiter: char) -> Option<(&'a str, &'a str)> {
    match input.find(delimiter) {
        Some(position) => {
            return Some((&input[..position], &input[position + delimiter.len_utf8()..]))
        },
        None => return None
    }
}

pub fn common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str {
    a.char_indices()
        .zip(b.char_indices())
        .take_while(| ((_, ca), (_, cb)) | ca == cb )
        .map(| ((i, ca), _) | i + ca.len_utf8())
        .last()
        .map(|end| &a[..end])
        .unwrap_or("")
}
