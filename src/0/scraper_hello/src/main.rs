fn main() {
    let html = r#"
<html>
    <body>
        <div class="ssss"><ul><li name="nn">NotSelect</li></ul></div>
        <div class="some-list">
            <ul>
                <li name="n1">item1</li>
                <li          >item2</li>
                <li name="n3">item3</li>
            </ul>
        </div>
    </body>
</html>"#;
    let doc = scraper::Html::parse_document(html);
    let sel = scraper::Selector::parse("div.some-list ul li[name]").unwrap();
//    println!("{:?}", doc.select(&sel));
    for node in doc.select(&sel) {
        println!("{:?}", node.value().name());       // 要素名
        println!("{:?}", node.value().attr("name")); // 属性の取得
        println!("{:?}", node.inner_html());         // テキストノードの取得
        println!("{:?}", node.text().collect::<Vec<_>>());    // テキストノードの取得
        println!("{:?}", node.text().collect::<Vec<_>>()[0]); // テキストノードの取得
    }
}
