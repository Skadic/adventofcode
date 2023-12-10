import Control.Monad (liftM)
import Debug.Trace

parseInput :: String -> [[Integer]]
parseInput = map (map read . words) . lines

next :: [Integer] -> [Integer]
next l@(_ : xs) = zipWith (flip (-)) l xs

lasts :: [Integer] -> [Integer]
lasts [] = [0]
lasts v = last v : lastVals
  where
    nex = next v
    lastVals =
      if any (/= 0) nex
        then lasts nex
        else [0]

firsts :: [Integer] -> [Integer]
firsts [] = [0]
firsts v@(f : _) = f : lastVals
  where
    nex = next v
    lastVals =
      if any (/= 0) nex
        then firsts nex
        else [0]

main :: IO ()
main = do
  v <- readFile "input.txt"
  let ls = parseInput v
  let part1 = sum . map (sum . lasts) $ ls
  let part2 = sum . map (foldl (flip (-)) 0 . reverse . firsts) $ ls
  print $ "Part 1 " ++ show part1
  print $ "Part 2 " ++ show part2
