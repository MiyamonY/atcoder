open Batteries
module EnumL = Enum.Labels
module ListL = List.Labels
module ArrayL = Array.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = dbg0 x; x

let id = identity

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

let (h, w)  = scan "%d %d" Tuple.Tuple2.make
let g = scan_matrix h w "." id
let m = Array.make_matrix h w (-1)
let ds = [(-1,0);(1,0);(0,-1);(0,1)]

let rec bfs visited queue =
  if Queue.is_empty queue then ()
  else
    let (i, j, n) = Queue.pop queue in
    ListL.iter ds ~f:(fun (di, dj) ->
        let x, y = i + di, j + dj in
        if between 0 x h && between 0 y w && not visited.(x).(y)
           && g.(x).(y) = "." then
          (visited.(x).(y) <- true;
           m.(x).(y) <- (succ n);
           Queue.push (x,y, succ n) queue));
    bfs visited queue

let () =
  let visited = Array.make_matrix h w false in
  let q = Queue.create () in
  visited.(0).(0) <- true;
  m.(0).(0) <- 0;
  Queue.push (0,0,1) q;
  bfs visited q;
  let n = m.(h-1).(w-1) in
  (if n = -1 then -1
   else
     h*w -
     (ArrayL.map g
        ~f:(fun line ->
            ArrayL.filter line ~f:((=) "#")
            |> Array.length)
      |> Array.sum) - n)
  |> Printf.printf "%d\n"
