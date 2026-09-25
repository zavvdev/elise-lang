// TODO: Accept Vec<AstNode::Typedef> and build a HashMap:

// .typedef (User :Record<{
//                     "name"    :Str
//                     "age"     :Int
//                     "address" :Str}>)
//
// .typedef (Data :List<:User>)
//
//
// HashMap {
//     [Alias("User")]            => Record,
//     [Alias("User"), "name"]    => Str,
//     [Alias("User"), "age"]     => Int,
//     [Alias("User"), "address"] => Str,
//
//     [Alias("Data")]                           => List<Alias("User")>,
//     [Alias("Data"), AbstractIndex]            => Alias("User"),
//     [Alias("Data"), AbstractIndex, "name"]    => Str,
//     [Alias("Data"), AbstractIndex, "age"]     => Int,
//     [Alias("Data"), AbstractIndex, "address"] => Str,
// }
