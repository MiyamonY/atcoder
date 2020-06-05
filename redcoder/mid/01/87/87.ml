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

module UnionFind = struct
  type elem = {root: int; size: int}

  let empty n = Array.init n (fun i -> {root=i; size=1})

  let root t n =
    let rec aux c =
      if c = t.(c).root then c
      else let p = aux t.(c).root in
        t.(c) <- {t.(c) with root=p}; p in
    aux n

  let size t v = t.(root t v).size

  let unite t v0 v1 =
    let w0 = root t v0 in
    let w1 = root t v1 in
    let s = (size t w0)+(size t w1) in
    t.(w1) <- {root=w0; size=s};
    t.(w0) <- {t.(w0) with size=s}

  let same t v0 v1 = root t v0 = root t v1
end

let (n, m) = scan "%d %d" Tuple2.make
let ls = List.rev @@ scan_lines m "%d %d" Tuple2.make

let () =
  let uf = UnionFind.empty (succ n) in
  let rec solve n ans = function
    | [] -> ans
    | (a,b)::rest ->
      if UnionFind.same uf a b then solve n (n::ans) rest
      else
        let x = UnionFind.size uf a in
        let y = UnionFind.size uf b in
        UnionFind.unite uf a b;
        solve (n-x*y) (n::ans) rest
  in
  solve (n*(pred n)/2) [] ls
  |> List.map String.of_int
  |> String.join "\n"
  |> Printf.printf "%s\n"
