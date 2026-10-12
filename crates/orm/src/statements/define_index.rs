// Statement syntax
// DEFINE INDEX @name ON [ TABLE ] @table [ FIELDS | COLUMNS ] @fields
// 	[ UNIQUE | SEARCH ANALYZER @analyzer [ BM25 [(@k1, @b)] ] [ HIGHLIGHTS ] ]
//
// -- Make sure that email addresses in the user table are always unique
// DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;

use surrealdb::types::Table;

use crate::{Field, Ident, NumberLike, Render, Renderer, TableIndex, ValueLike};

use super::{Statement, impl_definition};

/// Define a new database index.
/// Just like in other databases, SurrealDB uses indexes to help optimize query performance.
/// An index can consist of one or more fields in a table and can enforce a uniqueness constraint.
///
/// Example:
/// ```rust
/// use umbriel::{*, statements::{define_index}};
/// # let alien = Table::from("alien");
/// # let name = Field::new("name");
/// # let age = Field::new("age");
/// # let email = Field::new("email");
/// # let dob = Field::new("dob");
///
/// let query = define_index("alien_index")
///                 .on_table(alien)
///                 .fields(&[age, name, email, dob])
///                 .unique();
///
/// assert_eq!(query.to_debug_sql(),
/// "DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;");
/// ```
pub fn define_index(index_name: impl Into<TableIndex>) -> DefineIndexStatement {
    DefineIndexStatement {
        index_name: index_name.into(),
        table: None,
        fields: vec![],
        columns: vec![],
        unique: false,
        search_analyzer: None,
    }
}

/// One or more indexed fields.
pub enum Columns {
    /// One field
    Field(Field),
    /// Several fields
    Fields(Vec<Field>),
}

/// One or more indexed fields.
pub type Fields = Columns;

impl From<Columns> for Vec<Field> {
    fn from(value: Columns) -> Self {
        match value {
            Columns::Field(field) => vec![field],
            Columns::Fields(fields) => fields,
        }
    }
}

impl From<Field> for Columns {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

impl<const N: usize> From<&[Field; N]> for Columns {
    fn from(value: &[Field; N]) -> Self {
        Self::Fields(value.to_vec())
    }
}

impl<const N: usize> From<[Field; N]> for Columns {
    fn from(value: [Field; N]) -> Self {
        Self::Fields(value.into())
    }
}

impl<const N: usize> From<[ValueLike; N]> for Columns {
    fn from(value: [ValueLike; N]) -> Self {
        Self::Fields(value.into_iter().map(Field::from_expr).collect())
    }
}

impl From<Vec<ValueLike>> for Columns {
    fn from(value: Vec<ValueLike>) -> Self {
        Self::Fields(value.into_iter().map(Field::from_expr).collect())
    }
}

impl From<Vec<Field>> for Columns {
    fn from(value: Vec<Field>) -> Self {
        Self::Fields(value)
    }
}

/// A statement for defining a database Index.
pub struct DefineIndexStatement {
    index_name: TableIndex,
    table: Option<Table>,
    fields: Vec<Field>,
    columns: Vec<Field>,
    unique: bool,
    search_analyzer: Option<SearchAnalyzer>,
}

impl DefineIndexStatement {
    /// Set the table where the index is defined.
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }

    /// Set the columns on the table where the index should be defined. This is alternative to
    /// fields just like in a relational database
    pub fn columns(mut self, columns: impl Into<Columns>) -> Self {
        self.columns.extend(Vec::<Field>::from(columns.into()));
        self
    }

    /// Set the fields on the table where the index should be defined. This is alternative to
    /// columns
    pub fn fields(mut self, fields: impl Into<Fields>) -> Self {
        self.fields.extend(Vec::<Field>::from(fields.into()));
        self
    }

    /// Set whether the field should be unique
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// Set the search analyzer for the index
    pub fn search_analyzer(mut self, search_analyzer: SearchAnalyzer) -> Self {
        self.search_analyzer = Some(search_analyzer);
        self
    }
}

impl Statement for DefineIndexStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("DEFINE INDEX ").push(&self.index_name);

        if let Some(table) = &self.table {
            out.push_str(" ON TABLE ").push(table);
        }

        if !self.fields.is_empty() {
            out.push_str(" FIELDS ").push(&self.fields);
        }

        if !self.columns.is_empty() {
            out.push_str(" COLUMNS ").push(&self.columns);
        }

        if self.unique {
            out.push_str(" UNIQUE");
        } else if let Some(search_analyzer) = &self.search_analyzer {
            out.push_str(" ").push(search_analyzer);
        }
    }
}

impl_definition!(DefineIndexStatement);

/// Scoring for search
pub enum Scoring {
    /// BestMatching25
    Bm25(NumberLike, NumberLike),
    /// VectorSearch
    Vs,
}

impl Render for Scoring {
    fn render(&self, out: &mut Renderer) {
        match self {
            Scoring::Bm25(k1, b) => {
                out.push_str("BM25 ").push(k1).push_str(" ").push(b);
            }
            Scoring::Vs => {
                out.push_str("VS");
            }
        }
    }
}

/// The full-text search options of an index.
pub struct SearchAnalyzer {
    analyzer: Ident,
    highlight: bool,
    scoring: Option<Scoring>,
    doc_ids_order: Option<NumberLike>,
    doc_lengths_order: Option<NumberLike>,
    postings_order: Option<NumberLike>,
    terms_order: Option<NumberLike>,
}

impl SearchAnalyzer {
    /// Enables `HIGHLIGHTS`.
    pub fn highlight(mut self) -> Self {
        self.highlight = true;
        self
    }

    /// Scores with BM25.
    pub fn bm25(mut self, k1: impl Into<NumberLike>, b: impl Into<NumberLike>) -> Self {
        self.scoring = Some(Scoring::Bm25(k1.into(), b.into()));
        self
    }

    /// Scores with vector search.
    pub fn vs(mut self) -> Self {
        self.scoring = Some(Scoring::Vs);
        self
    }

    /// Sets `DOC_IDS_ORDER`.
    pub fn doc_ids_order(mut self, doc_ids_order: impl Into<NumberLike>) -> Self {
        self.doc_ids_order = Some(doc_ids_order.into());
        self
    }

    /// Sets `DOC_LENGTHS_ORDER`.
    pub fn doc_lengths_order(mut self, doc_lengths_order: impl Into<NumberLike>) -> Self {
        self.doc_lengths_order = Some(doc_lengths_order.into());
        self
    }

    /// Sets `POSTINGS_ORDER`.
    pub fn postings_order(mut self, postings_order: impl Into<NumberLike>) -> Self {
        self.postings_order = Some(postings_order.into());
        self
    }

    /// Sets `TERMS_ORDER`.
    pub fn terms_order(mut self, terms_order: impl Into<NumberLike>) -> Self {
        self.terms_order = Some(terms_order.into());
        self
    }
}

impl Render for SearchAnalyzer {
    fn render(&self, out: &mut Renderer) {
        out.push_str("SEARCH ANALYZER ").push(&self.analyzer);

        if self.highlight {
            out.push_str(" HIGHLIGHTS");
        }

        if let Some(scoring) = &self.scoring {
            out.push_str(" ").push(scoring);
        }

        let orders = [
            (" DOC_IDS_ORDER ", &self.doc_ids_order),
            (" DOC_LENGTHS_ORDER ", &self.doc_lengths_order),
            (" POSTINGS_ORDER ", &self.postings_order),
            (" TERMS_ORDER ", &self.terms_order),
        ];
        for (keyword, order) in orders {
            if let Some(order) = order {
                out.push_str(keyword).push(order);
            }
        }
    }
}

/// Function to start building a SearchParams object with an analyzer
///
/// ```rust
/// use umbriel::{statements::search_analyzer};
///
/// search_analyzer("my_analyzer")
///    .highlight()
///    .bm25(1.2, 0.75)
///    .doc_ids_order(1)
///    .doc_lengths_order(1)
///    .postings_order(1)
///    .terms_order(1);
/// ```
pub fn search_analyzer(analyzer_name: impl Into<Ident>) -> SearchAnalyzer {
    SearchAnalyzer {
        analyzer: analyzer_name.into(),
        highlight: false,
        scoring: None,
        doc_ids_order: None,
        doc_lengths_order: None,
        postings_order: None,
        terms_order: None,
    }
}
