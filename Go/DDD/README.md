### Entity

- Can be uniquely identified
- Are mutable
- Are dumb (act as placeholders for info)

### Value Objects

- Don't have unique identifiers
- But are immutable

### Aggregates

Aggregates are formed by combining Entities and Value Objects. These have root identity (should have only one) which helps to identify the aggregates. 
Business logic added to aggregates. 