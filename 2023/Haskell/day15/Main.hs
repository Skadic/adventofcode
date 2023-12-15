import Data.Char (ord)
import Data.List (elemIndex)
import Debug.Trace

hashStr :: String -> Int
hashStr = foldl (\acc c -> (acc + ord c) * 17 `mod` 256) 0

splitOn :: (Eq a) => a -> [a] -> [[a]]
splitOn token [] = []
splitOn token ls = case elemIndex token ls of
  Nothing -> [ls]
  Just idx -> if l /= [] then l : splitOn token r else splitOn token r
    where
      (l, temp) = splitAt idx ls
      (_, r) = splitAt 1 temp

data Lens = Lens String Int deriving (Show)

data Operation = Insert Lens | Delete String deriving (Show)

tag :: Lens -> String
tag (Lens t _) = t

focalLength :: Lens -> Int
focalLength (Lens _ f) = f

type Box = [Lens]

boxes :: [Box]
boxes = [[] | i <- [0 .. 255]]

applyOperation :: Operation -> [Box] -> [Box]
applyOperation op [] = []
applyOperation op@(Insert (Lens tg _)) bxs = l ++ newBox : r
  where
    hash = hashStr tg
    (l, box : r) = splitAt hash bxs
    newBox = operate op box
applyOperation op@(Delete tg) bxs = l ++ newBox : r
  where
    hash = hashStr tg
    (l, box : r) = splitAt hash bxs
    newBox = operate op box

operate :: Operation -> Box -> Box
operate (Insert lens) box = insertIntoBox lens box
operate (Delete tg) box = filter ((/= tg) . tag) box

insertIntoBox :: Lens -> Box -> Box
insertIntoBox lens [] = [lens]
insertIntoBox lens (b : bx)
  | tag lens == tag b = lens : bx
  | otherwise = b : insertIntoBox lens bx

parseOp :: String -> Operation
parseOp s = case elemIndex '=' s of
  Just i ->
    let (tg, _ : r) = splitAt i s
     in Insert $ Lens tg (read r)
  Nothing -> Delete $ init s

evalPower :: [Box] -> Int
evalPower = sum . zipWith (*) [1..] . map evalBox
  where
    evalBox box = sum $ zipWith (*) [1..] $ map focalLength box


main :: IO ()
main = do
  input <- readFile "input.txt"
  let list = splitOn ',' $ init input
  print $ "Part 1: " ++ show (sum $ map hashStr list)

  let ops = map parseOp list
  let finalBoxes = foldl (flip applyOperation) boxes ops
  print $  "Part 2: " ++ show (evalPower finalBoxes)
