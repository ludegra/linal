use super::Matrix;

impl<const M: usize, const N: usize, T> std::fmt::Display for Matrix<M, N, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut padding = [0; N];

        for row in self.matrix.iter() {
            for (i, value) in row.iter().enumerate() {
                padding[i] = std::cmp::max(padding[i], value.to_string().len() + 1);
            }
        }

        for (i, row) in self.matrix.iter().enumerate() {
            if i == 0 {
                write!(f, "⎡")?;
            } else if i == M - 1 {
                write!(f, "⎣")?;
            } else {
                write!(f, "⎢")?;
            }

            for (i, value) in row.iter().enumerate() {
                write!(f, "{:width$} ", value, width = padding[i])?;
            }

            if i == 0 {
                write!(f, "⎤")?;
            } else if i == M - 1 {
                write!(f, "⎦")?;
            } else {
                write!(f, "⎥")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
