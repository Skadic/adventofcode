let readLines filePath = System.IO.File.ReadLines(filePath)

let compartments (s: string) =
    let half = s.Length / 2
    s.Substring(0, half), s.Substring(half, half)

let dupLetter (left: string) (right: string) =
    let sortLeft = left |> Seq.sort |> Seq.toList
    let sortRight = right |> Seq.sort |> Seq.toList

    let rec check lefts rights =
        match lefts, rights with
        | l :: ls, r :: _ when l < r -> check ls rights
        | l :: _, r :: rs when r < l -> check lefts rs
        | l :: _, _ -> l
        | _, _ -> '\000'

    check sortLeft sortRight

let dupLetter3 (left: string) (middle: string) (right: string) =
    let sortLeft = left |> Seq.sort |> Seq.toList
    let sortMid = middle |> Seq.sort |> Seq.toList
    let sortRight = right |> Seq.sort |> Seq.toList

    let rec check lefts mids rights =
        match lefts, mids, rights with
        | l :: ls, m :: _, r :: _ when l < max m r -> check ls mids rights
        | l :: _, m :: ms, r :: _ when m < max l r -> check lefts ms rights
        | l :: _, m :: _, r :: rs when r < max l m -> check lefts mids rs
        | l :: _, _, _ -> l
        | _, _, _ -> '\000'

    check sortLeft sortMid sortRight

let uncurry f (a, b) = f a b

let charPrio c =
    let value = int c

    if c >= 'a' && c <= 'z' then
        value - (int 'a') + 1
    else
        value - (int 'A') + 27

let part1 =
    "input.txt"
    |> readLines
    |> Seq.map compartments
    |> Seq.map (uncurry dupLetter)
    |> Seq.map charPrio
    |> Seq.sum

let part2 =
    "input.txt"
    |> readLines
    |> Seq.chunkBySize 3
    |> Seq.map (fun arr -> dupLetter3 arr[0] arr[1] arr[2])
    |> Seq.map charPrio
    |> Seq.sum


printfn "Part 1: %d" part1
printfn "Part 2: %d" part2
