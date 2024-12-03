import Data.List(sort)

main :: IO ()
main = do
    input <- readFile "input.txt"
    let numbers = unzip $ map (toTuple . map read . words) (lines input) :: ([Int], [Int])
    putStrLn $ "Solution for Day 1 -- Part 1: " ++ show (part1 numbers)
    putStrLn $ "Solution for Day 1 -- Part 2: " ++ show (part2 numbers)

part1 :: ([Int], [Int]) -> Int
part1 (left, right) = sum $ zipWith (curry (abs . uncurry (-))) (sort left) (sort right)

part2 :: ([Int], [Int]) -> Int
part2 (left, right) = sum $ map (\x -> x * length (filter (x ==) right)) left

toTuple :: [a] -> (a, a)
toTuple [a, b] = (a, b)
