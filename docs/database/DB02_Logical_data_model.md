### Logical data model

After the conceptual data model design, we want to translate the EER model to relations, which we can optimize a bit in 
the process.

We start with the company-related entities, where we translate the **Company**'s specializations **Retailer** and 
**Brand** to their own relations, because we want to have companies own each other no matter if they are retailers or 
brands[^1]. Therefore, we need another relation **CompanyOwnership** to realize the owner-ownee relationship[^2].

<pre>
<b>Company</b>(<u>company_id</u>, company_name, locality, admin_area, country)
<b>Retailer</b>(<u><i>retailer_id</i></u>, market_share, annual_revenue,
         profit_margin, working_currency)
    FK: retailer_id ◊ Company(company_id)
<b>Brand</b>(<u><i>brand_id</i></u>, product_line)
    FK: brand_id ◊ Company(company_id)

<b>CompanyOwnership</b>(<u><i>owner_id</i>, <i>ownee_id</i></u>)
    FK: owner_id ◊ Company(company_id)
    FK: ownee_id ◊ Company(company_id)
</pre>

Furthermore, we can translate the article-specific entities **Category** and **Article** without many modifications, 
expect that minimize the amount of relations by putting the brand and category as referential attributes in the 
**Article** relation. The **ArticleVariant** is existentially dependent on the former relation and therefore becomes 
part of the relation's primary key.

<pre>
<b>Category</b>(<u>category_id</u>, name, description)

<b>Article</b>(<u>article_id</u>, name, description, origin_country, image_url,
        <i>brand_id</i>, <i>category_id</i>)
    FK: brand_id ◊ Brand(brand_id)
    FK: category_id ◊ Category(category_id)

<b>ArticleVariant</b>(<u><i>article_id</i>, variant_id</u>, unit, quantity, weightable)
    FK: article_id ◊ Article(article_id)
</pre>

Now, we can focus on the store-related entities. We need a relation **Store**, which is only tightly coupled by a 
reference to its retailer, because the stores will be populated mostly by hand for the current platform's goals, and it 
reduces the amount of attributes in further relations.

<pre>
<b>Store</b>(<u>store_id</u>, <i>retailer_id</i>, street_address, postal_code,
      locality, admin_area, country, url_address)
    FK: retailer_id ◊ Retailer(company_id)
</pre>

The stores all have their own article catalog and category list, which we want to realize by translating the 
StoreArticle and StoreCategory entities to the relations **StoreArticleMap** and **StoreCategoryMap**, which also 
includes the article variant and category respectively by a referential key attribute. Now, we have a mapping where we 
can populate the store-specific articles to our own internal article catalog and are able to categorize them 
automatically with an algorithm that is due to the application implementation.

<pre>
<b>StoreArticleMap</b>(<u><i>store_id</i>, store_article_id</u>, <i>article_id, variant_id</i>,
                since)
    FK: store_id ◊ Store(store_id)
    FK: (article_id, variant_id) ◊
        ArticleVariant(article_id, variant_id)

<b>StoreCategoryMap</b>(<u><i>store_id</i>, store_category_id</u>, <i>category_id</i>)
    FK: store_id ◊ Store(store_id)
    FK: category_id ◊ Category(category_id)
</pre>

At last, we can finally translate the historical and current price relationships to the **ArticlePriceHistory** and 
**ArticlePriceCurrent** relations, which only differ by an additional `date_from` for the cross product of the primary 
key in the first relation. As the read operations will surpass the insert operations by multiple orders of magnitude, 
we can introduce this redundancy to make the lookups for the current price data faster and only fetch the historical 
price data if needed or in a later point in time[^3].

<pre>
<b>ArticlePriceHistory</b>(<u><i>store_id</i>, <i>article_id, variant_id</i>, date_from</u>,
                    price_currency, price_value)
    FK: store_id ◊ Store(store_id)
    FK: (article_id, variant_id) ◊
        ArticleVariant(article_id, variant_id)

<b>ArticlePriceCurrent</b>(<u><i>store_id</i>, <i>article_id, variant_id</i></u>,
                    price_currency, price_value)
    FK: store_id ◊ Store(store_id)
    FK: (article_id, variant_id) ◊
        ArticleVariant(article_id, variant_id)
</pre>

[^1]: This can be optimized by having a relationship between retailer and company only. This is not part of the current 
design as this project was completed in a university course with specific design requirements, such as a binary 
recursive relationship. I want to improve this in a later revision.
[^2]: We don't care about circular ownerships for now. They should be impossible, but as ownerships will be populated 
mostly by hand, we can assume that this constraint is not important.
[^3]: This could be improved at a later point by either a (materialized) view or another form of cache. Also, the 
historical price relation should probably be partitioned in the physical data model to further optimize the query speed 
for lookups in specific time intervals (e.g. 1 month, 3 months, 6 months, 1 year).
