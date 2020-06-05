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

let (m, n) = scan "%d %d" Tuple2.make
let k = scan "%d" id
let ls =  scan_lines m "%s" id

let () =
  let mat0 = Array.make_matrix (succ m) (succ n) 0 in
  let mat1 = Array.make_matrix (succ m) (succ n) 0 in
  let mat2 = Array.make_matrix (succ m) (succ n) 0 in
  ListL.iteri ls ~f:(fun i a ->
      String.fold_lefti (fun (x0,y0,z0) j c ->
          let (x0,y0,z0) =
            match c with
            | 'J' -> (succ x0,y0,z0)
            | 'O' -> (x0,succ y0,z0)
            | 'I' -> (x0,y0,succ z0)
            | _ -> (x0,y0,z0) in
          let (x1,y1,z1) = mat0.(i).(succ j), mat1.(i).(succ j), mat2.(i).(succ j) in
          mat0.(succ i).(succ j) <- x0+x1;
          mat1.(succ i).(succ j) <- y0+y1;
          mat2.(succ i).(succ j) <- z0+z1;
          (x0,y0,z0)) (0,0,0) a |> ignore);
  ListL.iter (0 ++^ k) ~f:(fun _ ->
      let (a,b,c,d) = scan "%d %d %d %d" Tuple4.make in
      let calc m = m.(c).(d) - m.(pred a).(d) - m.(c).(pred b) + m.(pred a).(pred b) in
      let (x,y,z) = calc mat0, calc mat1, calc mat2 in
      Printf.printf "%d %d %d\n" x y z)
