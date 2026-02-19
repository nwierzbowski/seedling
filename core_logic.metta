(Alias &Thing "Thing")
(Alias &Relationship "Relationship")
(Alias &User "User")

(: &Thing Type)
(: &Relationship Type)

(: Instance Type)
(: Link Type)

(= 
    (add_symbol $child $child-alias $parent)
    (case
        (match &self (Alias $parent $any) Found)
        (
            (Found (
                    superpose (
                        (add-atom &self (: $child $parent))
                        (add-atom &self (Alias $child $child-alias))
                        (add-atom &self (: $child &Thing))
                    )
            ))
    (Empty (Error $parent "Parent symbol was not found, make sure it is added as a child to an existing symbol before trying again."))
)
)
)

(= 
    (add_link $rel $rel-alias $parent)
    (case
        (match &self (Alias $parent $any) Found)
        (
            (Found (
                    superpose (
                        (add-atom &self (: $rel $parent))
                        (add-atom &self (Alias $rel $rel-alias))
                        (add-atom &self (: $rel &Relationship))
                    )
            ))
    (Empty (Error $parent "Parent relationship was not found, make sure it is added as a child to an existing relationship before trying again."))
)
)
)

(= 
    (relate $rel $host1 $host2)
    (case
        (match &self (: $host1 &Thing) Found)
        (
            (Found (case
                    (match &self (: $host2 &Thing) Found)
                    (
                        (Found (case
                                (match &self (: $rel &Relationship) Found)
                                (
                                    (Found (
                                            superpose (
                                                (add-atom &self ($rel $host1 $host2))
                                            )

                                    ))
                            (Empty (Error $rel "rel relationship was not found, make sure it is added via add_link before trying again."))
                        )
                ))

        (Empty (Error $host "host2 symbol was not found, make sure it is added via add_symbol before trying again."))
    )
))
(Empty (Error $host "host1 symbol was not found, make sure it is added via add_symbol before trying again."))
)
)
)