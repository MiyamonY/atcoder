open Batteries
module EnumL = Enum.Labels
module ListL = List.Labels
module ArrayL = Array.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = dbg0 x; x

let id = identity

let (+) a b = a + b mod 10000

let (++) n m = List.range n `To m
let (++^) n m = if n < m then List.range n `To (pred m) else []

let scan fmt = Scanf.sscanf (read_line ()) fmt

let scan_lines n fmt f =
  if n = 0 then Enum.empty ()
  else
    List.map (fun _ -> scan fmt f) (1 ++ n)
    |> List.enum

let scan_list sep cnv =
  let line = read_line () in
  (match sep with
   | None -> List.map String.of_char @@ String.to_list line
   | Some sep -> String.split_on_char sep line)
  |> List.map cnv

let scan_array ?sep cnv = Array.of_list @@ scan_list sep cnv

let scan_matrix n m e ?sep conv =
  let arr = Array.make_matrix n m e in
  ListL.iter (0 ++^ n)
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list sep conv);
  arr

let rec powerset e =
  match Enum.get e with
  | None -> Enum.singleton @@ Enum.empty ()
  | Some v ->
    let f = powerset e in
    let g = Enum.clone f in
    EnumL.map f ~f:(fun x -> let y = Enum.clone x in push y v; y)
    |>  Enum.append g

let permutations l =
  let rec aux l =
    let rec interleave x = function
      | [] -> [[x]]
      | (hd::tl) as lst ->
        (x::lst) ::
        (ListL.map ~f:(List.cons hd)  @@ interleave x tl)
    in
    match l with
    | [] -> [[]]
    | hd::tl -> List.concat @@ List.map (interleave hd) @@ aux tl in
  let l = List.sort (List.compare Int.compare) @@ aux @@ List.of_enum l in
  List.enum % ListL.map ~f:List.enum @@ l

let intersection l =
  EnumL.filter ~f:(fun x -> EnumL.exists ~f:((=) x) @@ Enum.clone l)

let lower_bound n f =
  let rec aux l u =
    if u - l > 1 then
      let m = (l + u) / 2 in
      if f m then aux l m
      else aux m u
    else u in aux (-1) n

let between n x m = n <= x && x < m

let (n, k) = scan "%d %d" Tuple.Tuple2.make
let arr = let arr = Array.make (succ n) 0 in
  ListL.iter (0 ++^ k)
    ~f:(fun _ -> let (a,b) = scan "%d %d" Tuple.Tuple2.make in
         arr.(a) <- b);
  arr

let () =
  let dp = Bigarray.Array3.create Bigarray.Int Bigarray.C_layout (n+1) 4 4  in
  if arr.(1) <> 0 && arr.(2) <> 0 then
    dp.{2, arr.(2), arr.(1)} <- 1
  else if arr.(2) <> 0 then      (* ex. arr.(2) = 3 *)
    ListL.iter (1 ++ 3) ~f:(fun i ->
        dp.{2, arr.(2), i} <- 1)
  else if arr.(1) <> 0 then       (* ex. arr.(1) = 1 *)
    ListL.iter (1 ++ 3) ~f:(fun i->
        dp.{2, i, arr.(1)} <- 1)
  else
    ListL.iter (1 ++ 3) ~f:(fun i ->
        ListL.iter (1++3) ~f:(fun j -> dp.{2,i,j} <-1 ));
  ListL.iter (3 ++ n) ~f:(fun i ->
      if arr.(i) = 1 || arr.(i) = 0 then
        begin
          dp.{i,1,1} <- (               dp.{i-1,1,2} + dp.{i-1,1,3}) mod 10000;
          dp.{i,1,2} <- (dp.{i-1,2,1} + dp.{i-1,2,2} + dp.{i-1,2,3}) mod 10000;
          dp.{i,1,3} <- (dp.{i-1,3,1} + dp.{i-1,3,2} + dp.{i-1,3,3}) mod 10000;
        end;
      if arr.(i) = 2 || arr.(i) = 0 then
        begin
          dp.{i,2,1} <- (dp.{i-1,1,1} + dp.{i-1,1,2} + dp.{i-1,1,3}) mod 10000;
          dp.{i,2,2} <- (dp.{i-1,2,1} +              + dp.{i-1,2,3}) mod 10000;
          dp.{i,2,3} <- (dp.{i-1,3,1} + dp.{i-1,3,2} + dp.{i-1,3,3}) mod 10000;
        end;
      if arr.(i) = 3 || arr.(i) = 0 then
        begin
          dp.{i,3,1} <- (dp.{i-1,1,1} + dp.{i-1,1,2} + dp.{i-1,1,3}) mod 10000;
          dp.{i,3,2} <- (dp.{i-1,2,1} + dp.{i-1,2,2} + dp.{i-1,2,3}) mod 10000;
          dp.{i,3,3} <- (dp.{i-1,3,1} + dp.{i-1,3,2}               ) mod 10000;
        end
    );
  ListL.map (1 ++ 3) ~f:(fun i ->
      ListL.map (1++3) ~f:(fun j ->
          dp.{n,i,j}
        )
    )
  |> List.concat
  |> ListL.fold_left ~init:0 ~f:(fun a i -> (a+i) mod 10000 )
  |> Printf.printf "%d\n"
