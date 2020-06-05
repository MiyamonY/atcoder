module ArrayL = ArrayLabels
module ListL = ListLabels

let dbg = Printf.printf "[debug]%s"

let max_num = 100_000_000_000

let id = fun x -> x
let tuple2 x y = (x,y)
let tuple3 x y z = (x,y,z)
let succ x = x + 1
let pred x = x - 1

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let scan fmt f = Scanf.sscanf (read_line ()) fmt f

let scan_list f =
  read_line ()
  |> Str.split (Str.regexp " +")
  |> ListL.map ~f:f

let scan_lines n fmt f =
  List.map (fun _ -> scan fmt f) (0++^n)

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr; arr

let between n x m = n <= x && x < m

let string_to_list s =
  List.map (String.get s) (0 ++^ String.length s)

module UnionFind = struct
  let empty n = Array.init n id

  let root t n =
    let rec aux c =
      if c = t.(c) then c
      else let p = aux t.(c) in t.(c) <- p; p in
    aux n

  let unite t v0 v1 =
    let w0 = root t v0 in
    let w1 = root t v1 in
    t.(w1) <- w0

  let same t v0 v1 =
    root t v0 = root t v1
end

let (n, q) =scan "%d %d" tuple2
let uf = UnionFind.empty n

let () =
  ListL.iter (0 ++^ q) ~f:(fun _ ->
      let (q, a, b) = scan "%d %d %d" tuple3 in
      if q = 0 then
        UnionFind.unite uf a b
      else
      if UnionFind.same uf a b then
        Printf.printf "1\n"
      else Printf.printf "0\n")
