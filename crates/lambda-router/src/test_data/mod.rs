pub const SAMPLE_SUPERGRAPH: &str = r#"
schema @core(feature: "https://specs.apollo.dev/core/v0.1") @core(feature: "https://specs.apollo.dev/join/v0.1") @apollo_studio_metadata(launchId: "420b139d-573e-45c8-80ab-101b67e200d2", buildId: "420b139d-573e-45c8-80ab-101b67e200d2", checkId: null) {
  query: Query
}

directive @core(feature: String!) repeatable on SCHEMA

directive @join__field(graph: join__Graph, requires: join__FieldSet, provides: join__FieldSet) on FIELD_DEFINITION

directive @join__type(graph: join__Graph!, key: join__FieldSet) repeatable on OBJECT | INTERFACE

directive @join__owner(graph: join__Graph!) on OBJECT | INTERFACE

directive @join__graph(name: String!, url: String!) on ENUM_VALUE

type Customer @join__owner(graph: CUSTOMERS) @join__type(graph: CUSTOMERS, key: "id") @join__type(graph: REVIEWS, key: "id") {
  id: ID! @join__field(graph: CUSTOMERS)
  username: String! @join__field(graph: CUSTOMERS)
  reviews: [Review!]! @join__field(graph: REVIEWS)
}

scalar join__FieldSet

enum join__Graph {
  CUSTOMERS @join__graph(name: "customers", url: "https://mesh.ocrateris.cloud/service/customers")
  PRODUCTS @join__graph(name: "products", url: "https://mesh.ocrateris.cloud/service/products")
  REVIEWS @join__graph(name: "reviews", url: "https://mesh.ocrateris.cloud/service/reviews")
}

type Product @join__owner(graph: PRODUCTS) @join__type(graph: PRODUCTS, key: "upc") @join__type(graph: REVIEWS, key: "upc") {
  upc: String! @join__field(graph: PRODUCTS)
  name: String! @join__field(graph: PRODUCTS)
  price: Int! @join__field(graph: PRODUCTS)
  reviews: [Review!]! @join__field(graph: REVIEWS)
}

type Query {
  listProducts: [Product!]! @join__field(graph: PRODUCTS)
  me: Customer! @join__field(graph: CUSTOMERS)
}

type Review {
  body: String!
  author: Customer!
  product: Product!
}

directive @apollo_studio_metadata(launchId: String, buildId: String, checkId: String) on SCHEMA
"#;
