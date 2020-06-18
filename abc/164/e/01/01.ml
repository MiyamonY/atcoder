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

let (n,m,s) = scan "%d %d %d" Tuple3.make
let ls = scan_lines m "%d %d %d %d" Tuple4.make
let ms = Array.of_list @@ scan_lines n "%d %d" Tuple2.make

let num = 2500

let () =
  let mat = Array.make n [] in
  ListL.iter ls ~f:(fun (u,v,a,b) ->
      mat.(pred u) <- (pred v, a, b)::mat.(pred u);
      mat.(pred v) <- (pred u, a, b)::mat.(pred v));
  let dp = Array.make_matrix n (succ num) Int.max_num in
  let rec aux i h =
    if Heap.size h = 0 then ()
    else
      let (dist, u, s) = Heap.find_min h in
      if dist > dp.(u).(s) then aux i @@ Heap.del_min h
      else
        (let (c,d) = ms.(u) in
         let h = if s + c <= num && d + dist < dp.(u).(s+c) then
             (dp.(u).(s+c) <- d+dist;
              Heap.del_min h |> Heap.add (d+dist, u, s+c))
           else Heap.del_min h in
         aux (succ i) @@
         ListL.fold_left mat.(u) ~init:h ~f:(fun h (v,a,b) ->
             if s - a >= 0 then
               let newdist = dist + b in
               if newdist < dp.(v).(s-a) then
                 (dp.(v).(s-a) <- newdist;
                  Heap.add (newdist, v, s-a) h)
               else h
             else h))
  in
  dp.(0).(min s 2500) <- 0;
  aux 0 (Heap.empty |> Heap.add (0, 0, min 2500 s));
  (* Array.print ~sep:"\n" (Array.print Int.print) stderr dp; *)
  ListL.iter (1++^n) ~f:(fun i ->
      Printf.printf "%d\n" @@ Array.min dp.(i))
