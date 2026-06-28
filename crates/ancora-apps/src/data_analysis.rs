/// Data analysis application.
///
/// Provides offline statistical summaries and simple tabular analysis
/// over in-memory datasets.

#[derive(Debug, Clone)]
pub struct DataSet {
    pub name: String,
    columns: Vec<String>,
    rows: Vec<Vec<f64>>,
}

impl DataSet {
    pub fn new(name: impl Into<String>, columns: Vec<String>) -> Self {
        Self {
            name: name.into(),
            columns,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<f64>) -> Result<(), String> {
        if row.len() != self.columns.len() {
            return Err(format!(
                "expected {} columns, got {}",
                self.columns.len(),
                row.len()
            ));
        }
        self.rows.push(row);
        Ok(())
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c == name)
    }

    pub fn column_values(&self, col: usize) -> Vec<f64> {
        self.rows.iter().map(|r| r[col]).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub column: String,
    pub count: usize,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub variance: f64,
}

pub struct DataAnalyzer;

impl DataAnalyzer {
    pub fn summarise(dataset: &DataSet, column_name: &str) -> Result<ColumnStats, String> {
        let col_idx = dataset
            .column_index(column_name)
            .ok_or_else(|| format!("column '{}' not found", column_name))?;
        let values = dataset.column_values(col_idx);
        if values.is_empty() {
            return Err("no data".to_string());
        }
        let count = values.len();
        let sum: f64 = values.iter().sum();
        let mean = sum / count as f64;
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count as f64;
        Ok(ColumnStats {
            column: column_name.to_string(),
            count,
            mean,
            min,
            max,
            variance,
        })
    }

    /// Return the top-N rows by a given column value (descending).
    pub fn top_n(dataset: &DataSet, column_name: &str, n: usize) -> Result<Vec<Vec<f64>>, String> {
        let col_idx = dataset
            .column_index(column_name)
            .ok_or_else(|| format!("column '{}' not found", column_name))?;
        let mut rows = dataset.rows.clone();
        rows.sort_by(|a, b| b[col_idx].partial_cmp(&a[col_idx]).unwrap_or(std::cmp::Ordering::Equal));
        Ok(rows.into_iter().take(n).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarise_column() {
        let mut ds = DataSet::new("sales", vec!["revenue".to_string(), "units".to_string()]);
        ds.add_row(vec![100.0, 10.0]).unwrap();
        ds.add_row(vec![200.0, 20.0]).unwrap();
        let stats = DataAnalyzer::summarise(&ds, "revenue").unwrap();
        assert!((stats.mean - 150.0).abs() < 1e-9);
        assert!((stats.min - 100.0).abs() < 1e-9);
        assert!((stats.max - 200.0).abs() < 1e-9);
    }
}
