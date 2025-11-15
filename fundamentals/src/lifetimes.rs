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
    Some(&s[start..end])
}
