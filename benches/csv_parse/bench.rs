///Baseline: Simple csv with just one entry.
///Rows: 1 header, 100 rows
///Headers: 100 headers, 1 row

#[macro_use]
extern crate bencher;

static BASELINE: &str = r#"Col
Data"#;

static ROWS: &str = r#"Col1
Row1
Row2
Row3
Row4
Row5
Row6
Row7
Row8
Row9
Row10
Row11
Row12
Row13
Row14
Row15
Row16
Row17
Row18
Row19
Row20
Row21
Row22
Row23
Row24
Row25
Row26
Row27
Row28
Row29
Row30
Row31
Row32
Row33
Row34
Row35
Row36
Row37
Row38
Row39
Row40
Row41
Row42
Row43
Row44
Row45
Row46
Row47
Row48
Row49
Row50
Row51
Row52
Row53
Row54
Row55
Row56
Row57
Row58
Row59
Row60
Row61
Row62
Row63
Row64
Row65
Row66
Row67
Row68
Row69
Row70
Row71
Row72
Row73
Row74
Row75
Row76
Row77
Row78
Row79
Row80
Row81
Row82
Row83
Row84
Row85
Row86
Row87
Row88
Row89
Row90
Row91
Row92
Row93
Row94
Row95
Row96
Row97
Row98
Row99
Row100"#;

static HEADERS: &str = r#"Col1,Col2,Col3,Col4,Col5,Col6,Col7,Col8,Col9,Col10,Col11,Col12,Col13,Col14,Col15,Col16,Col17,Col18,Col19,Col20,Col21,Col22,Col23,Col24,Col25,Col26,Col27,Col28,Col29,Col30,Col31,Col32,Col33,Col34,Col35,Col36,Col37,Col38,Col39,Col40,Col41,Col42,Col43,Col44,Col45,Col46,Col47,Col48,Col49,Col50,Col51,Col52,Col53,Col54,Col55,Col56,Col57,Col58,Col59,Col60,Col61,Col62,Col63,Col64,Col65,Col66,Col67,Col68,Col69,Col70,Col71,Col72,Col73,Col74,Col75,Col76,Col77,Col78,Col79,Col80,Col81,Col82,Col83,Col84,Col85,Col86,Col87,Col88,Col89,Col90,Col91,Col92,Col93,Col94,Col95,Col96,Col97,Col98,Col99,Col100
Data1,Data2,Data3,Data4,Data5,Data6,Data7,Data8,Data9,Data10,Data11,Data12,Data13,Data14,Data15,Data16,Data17,Data18,Data19,Data20,Data21,Data22,Data23,Data24,Data25,Data26,Data27,Data28,Data29,Data30,Data31,Data32,Data33,Data34,Data35,Data36,Data37,Data38,Data39,Data40,Data41,Data42,Data43,Data44,Data45,Data46,Data47,Data48,Data49,Data50,Data51,Data52,Data53,Data54,Data55,Data56,Data57,Data58,Data59,Data60,Data61,Data62,Data63,Data64,Data65,Data66,Data67,Data68,Data69,Data70,Data71,Data72,Data73,Data74,Data75,Data76,Data77,Data78,Data79,Data80,Data81,Data82,Data83,Data84,Data85,Data86,Data87,Data88,Data89,Data90,Data91,Data92,Data93,Data94,Data95,Data96,Data97,Data98,Data99,Data100"#;

mod _csv;

benchmark_group!(baseline, _csv::baseline);
benchmark_group!(rows, _csv::rows);
benchmark_group!(headers, _csv::headers);

benchmark_main!(baseline, rows, headers);
