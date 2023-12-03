## Conceptual data model

The platform will refer to the supermarket chains as _retailers_, which are companies, and each retailer runs a set of 
_stores_. Each store runs a catalog of their own _store articles_ and describes some _store categories_ for them. The 
store article are mapped to the platforms internal catalog, where the store articles are aggregated to _articles_, 
which are divided into different _article variants_ for different units and quantities. The store categories are mapped 
to the platforms internal _category_ list. Stores periodically change the prices for some article variants, where the
most recent price is the effective current price of the article variant. Lastly, an article is branded with a _brand_, 
which is - similar to a retailer - a _company. An article also has a single _category_ that describes it.

### Entities

- A **company** is an entity type with a short name and a rough geographical location.
- A **retailer** is a specialization of a company, that has a (regional) market share, an annual revenue, a profit 
  margin in a given working currency.
- A **brand** is a specialization of a company, that has a main production line industry (e.g. cosmetics).
- A **store** is an entity type with a precise geographical location or a URL address, where the latter should be a URL 
  to the online store of that retailer. No matter the address type, it should specify the country in which the store 
  operates in.
- An **article** is an entity type with a reasonable short name, a description, an origin country and a image.
- An **article variant** is a weak entity type dependent on an article that has a quantity of a fixed unit. It is 
  possible for an article variant to be weightable, that is its priced variably.
- A **category** is an entity type with a reasonable aggregated name and a description.
- A **store article** is a weak entity type dependent on a store.
- A **store category** is a weak entity type dependent on a store.

### Relationships

- A company may be _owned_ by multiple other companies.
- A retailer _runs_ multiple stores.
- An article _has_ multiple article variants of their own.
- Multiple stores _change prices for_ multiple article variants, where we store the price value, the price currency,
  and the date and time when this price became effective.
- Multiple article variants _are currently sold at_ specific prices at multiple stores. They derive the current prices 
  from the most recent price in the former relationship.
- Multiple articles _are branded with_ a brand.
- Multiple article _are part of_ a category.
- Multiple stores _catalog_ many store articles, that _are mapped to_ the platforms internal article variants.
- Multiple stores _describe_ many store categories, that _are mapped to_ the platforms internal categories.

### EER model

![EER model](https://github.com/dakralex/pricevista/blob/master/docs/database/eer_model.svg)
