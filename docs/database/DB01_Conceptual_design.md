## Conceptual design

The platform will refer to the supermarket chains as _retailers_, which have their individual _stores_. They have a
catalog of _articles_, which have a _brand_ that is owned by a single _manufacturer_, that may or may not be linked to
be a store brand. To facilitate the linkage between ownership, retailers, and brands, we will generalize both retailers
and manufacturers as _companies_ respectively, that may own or share something of each other. Additionally, to enable
associations between identical articles with different unit sizes and packages, we will create a mapping in our own
_article catalog_, where each article can have multiple representations known as _article variants_.

### Entities

- A **company** is an entity type with a unique identifier, a short name, and a geographical location.
- A **retailer** is a specialization of a company, that also has a market share within their legal bounds, a revenue
  and a net profit margin in a given currency.
- A **manufacturer** is a specialization of a company, that also has a main production line industry associated with
  them (e.g. dairy products, cosmetics).
- A **store** is an entity type with a unique identifier, a geographical location or an internet address, where the
  latter should be a URL to the online store of that retailer. No matter the address type, it should specify the country
  in which the store operates in.
- An **article** is an entity type with a unique identifier, a reasonable name, and an optional description.
- An **article variant** is a weak entity type with a unique identifier, the measure unit and the quantity for that
  unit. It is possible for an article variant to be _weightable_, that is there is no fixed quantity and the price is
  per unit.
- A **brand** is an entity type with a unique identifier, that has a name attached to it.
- A **category** is an entity type with a unique identifier, that has a name and a description attached to it.

### Relationships

- A company may be _shared_ by multiple other companies.
- A retailer _runs_ multiple stores.
- A manufacturer _owns_ multiple brands.
- An article _has_ multiple article variants of their own.
- Multiple stores _change prices for_ multiple article variants, where we store the price value, the price currency,
  and the date and time when this price became effective.
- An article variant _is currently sold_ at a specific price at multiple stores. They derive the current prices from
  the price changes relationship.
- Multiple articles _are branded with_ a brand.
- Multiple article _are part of_ a category.

### EER model

![EER model](https://github.com/dakralex/pricevista/blob/master/docs/database/eer_model.svg)
