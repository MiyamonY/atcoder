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

module ModOp = struct
  let prime = 1_000_000_007
  let (-) a b = (a + prime - b) mod prime
  let (+) a b = (a + b) mod prime
  let ( * ) a b = (a * b) mod prime

  let rec pow x n =
    if n = 0 then 1
    else if n mod 2 = 0 then pow (x*x) (n/2)
    else x * pow (x*x) (n/2)
end

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

let n = scan "%d" id
let s = scan "%s" id

let () =
  let s = String.to_list s |> Array.of_list in
  let acc = Array.make n (0,0,0) in
  ArrayL.iteri s ~f:(fun i c ->
      let (r,g,b) = if i = 0 then (0,0,0) else acc.(pred i) in
      acc.(i) <- match c with
        | 'R' -> (succ r, g, b)
        | 'G' -> (r, succ g, b)
        | 'B' -> (r, g, succ b)
        | _ -> (r,g,b));
  ListL.map (1++^n) ~f:(fun j ->
      ListL.map (0++^j) ~f:(fun i ->
          let k = j + (j-i) in
          let same = if k < n && s.(i) <> s.(j) && s.(j) <> s.(k) && s.(k) <> s.(i) then -1 else 0 in
          let (r,g,b) = acc.(n-1) in
          let (r0,g0,b0) = acc.(j) in
          same + (match s.(i), s.(j) with
              | 'R', 'G' -> b-b0
              | 'R' ,'B' -> g-g0
              | 'G', 'R' -> b-b0
              | 'G', 'B' -> r-r0
              | 'B', 'R' -> g-g0
              | 'B', 'G' -> r-r0
              | _ -> 0)
        ) |> List.sum
    ) |> List.sum
  |> Printf.printf "%d\n"
