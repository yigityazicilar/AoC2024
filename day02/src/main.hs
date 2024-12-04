import Data.List (tails, sort)

main :: IO()
main = do
    input <- readFile "input.txt"
    let reports = map (map read . words) (lines input) :: [[Int]]
    putStrLn $ "Solution for Day 02 -- Part 1: " ++ show (length $ filter part1ReportValid reports)
    putStrLn $ "Solution for Day 02 -- Part 2: " ++ show (length $ filter part2ReportValid reports)

windows :: Int -> [a] -> [[a]]
windows n = foldr (zipWith (:)) (repeat []) . take n . tails

isSorted :: [Int] -> Bool
isSorted [] = True
isSorted [_] = True
isSorted (x:y:xs) = x <= y && isSorted (y:xs)

reverseList :: [Int] -> [Int]
reverseList [] = []
reverseList (x:xs) = reverseList xs ++ [x]

removeIndex :: Int -> [a] -> [a]
removeIndex _ [] = []
removeIndex n (x:xs)
    | n < 0 = x:xs
    | n == 0 = xs
    | otherwise = x:removeIndex (n - 1) xs

part1ReportValid :: [Int] -> Bool
part1ReportValid report = all (\[x, y] -> abs (x - y) >= 1 && abs (x - y) <= 3 && (isSorted report || isSorted (reverseList report))) (windows 2 report)

part2ReportValid :: [Int] -> Bool
part2ReportValid report = do
    let validByDefault = part1ReportValid report
    let validByRemoval = any part1ReportValid [removeIndex i report | i <- [0..length report - 1]]
    validByDefault || validByRemoval

