import qualified Data.Map as M
import Data.List
main = interact solve

solve x = winner ++ "\n"
    where (cand:input) = lines x
          numCand = read cand :: Int
          candidates = readCandidates (take (numCand * 2) input) M.empty
          (nv:votes) = drop (numCand * 2) input
          result = handleVotes votes $ M.fromList $ zip (M.keys candidates) $ repeat 0
          winner = if (count wscore $ M.elems result) > 1 then "tie" else candidates M.! wname
          (wname, wscore) = maximumBy (\x y -> (snd x) `compare` (snd y)) $ M.toList result

count x = length . filter (x==) 

readCandidates [] m = m
readCandidates (name:party:rest) m = readCandidates rest $ M.insert name party m

handleVotes [] m = m
handleVotes (vote:votes) m = handleVotes votes $ if M.member vote m then M.insertWith (+) vote 1 m
                                                 else m
