open Batteries
module EnumL = Enum.Labels
module ListL = List.Labels
module ArrayL = Array.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = dbg0 x; x

let id = identity

let (++) n m = List.range n `To m
let (++-) n m = if n >= m then List.range n `Downto m else []
let (++^) n m = if n < m then List.range n `To (pred m) else []

let scan fmt = Scanf.sscanf (read_line ()) fmt

let scan_lines n fmt f =
  if n = 0 then []
  else List.map (fun _ -> scan fmt f) (1 ++ n)

let scan_list ?sep cnv =
  let line = read_line () in
  (match sep with
   | None -> List.map String.of_char @@ String.to_list line
   | Some sep -> String.split_on_char sep line)
  |> List.map cnv

let scan_array ?sep cnv = Array.of_list @@ scan_list ?sep cnv

let scan_matrix n m e ?sep conv =
  let arr = Array.make_matrix n m e in
  ListL.iter (0 ++^ n)
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list ?sep conv);
  arr

let atoi c = Char.code c - Char.code '0'

let rec powerset = function
  | [] -> [[]]
  | hd::tl ->
    let pws =  powerset tl in
    pws @ ListL.map pws ~f:(fun pw -> hd::pw)

let permutations l =
  let rec interleave x = function
    | [] -> [[x]]
    | (hd::tl) as l ->
      (x::l) :: (interleave x tl |> ListL.map ~f:(fun l -> hd::l)) in
  let rec aux = function
    | [] -> [[]]
    | a::rest ->
      aux rest |> ListL.map ~f:(interleave a) |> List.concat in
  aux l

let intersection l =
  EnumL.filter ~f:(fun x -> EnumL.exists ~f:((=) x) @@ Enum.clone l)

let zip l m =
  let n = min (List.length l) (List.length m) in
  List.combine (List.take n l) (List.take n m)

let lower_bound n f =
  let rec aux l u =
    if u - l > 1 then
      let m = (l + u) / 2 in
      if f m then aux l m
      else aux m u
    else u in aux (-1) n

let between n x m = n <= x && x < m

let (n, m) = scan "%d %d" Tuple2.make
let ls = scan_lines m "%d %d" Tuple2.make

let () =
  let mat = Array.make (succ n) [] in
  ListL.iter ls ~f:(fun (a,b) ->
      mat.(a) <- b::mat.(a);
      mat.(b) <- a::mat.(b));
  let q = Queue.create () in
  Queue.add (1,1) q;
  let parents = Array.make (succ n) 0 in
  let rec wfs q =
    if Queue.is_empty q then ()
    else
      let (p, n) = Queue.pop q in
      if parents.(n) <> 0 then wfs q
      else
        (parents.(n) <- p;
         ListL.iter mat.(n) ~f:(fun m ->
             Queue.add (n, m) q);
         wfs q)
  in
  wfs q;
  if Array.exists ((=) 0)@@ Array.sub parents 2 (Array.length parents - 2)  then
    Printf.printf "No\n"
  else
    (Printf.printf "Yes\n";
     ListL.iter (2++n) ~f:(fun i ->
         Printf.printf "%d\n" parents.(i)))
