pub fn map<F, T, U>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut out = Vec::with_capacity(input.len());
    for item in input {
        out.push(function(item));
    }
    out
}