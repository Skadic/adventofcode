let readLines filePath = System.IO.File.ReadLines(filePath)

let splitInput lines: string list list =
    match lines with
    | [] -> []
    | lines ->
        let rec splitInputInternal remainingLines acc =
            match remainingLines with
            | [] -> [ acc ]
            | "" :: rest -> acc :: (splitInputInternal rest [])
            | x :: rest -> splitInputInternal rest (x :: acc)
        
        splitInputInternal lines []


let part1 =
    "input.txt"
    |> readLines
    |> Seq.toList
    |> splitInput
    |> List.map (List.map int)
    |> List.map (List.fold (+) 0)
    |> List.max

let part2 =
    "input.txt"
    |> readLines
    |> Seq.toList
    |> splitInput
    |> List.map (List.map int)
    |> List.map (List.fold (+) 0)
    |> List.sortDescending
    |> Seq.take 3
    |> Seq.sum

printfn "Part 1: %d" part1 
printfn "Part 2: %d" part2 