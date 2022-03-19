import qualified Data.Text as T
main = interact solve
solve line
    | l == r = "correct"
    | otherwise = "fix"
    where (l:r:_) = map (T.length) $ T.splitOn (T.pack "()") $ T.strip $ T.pack line