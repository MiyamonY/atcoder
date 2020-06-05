module ArrayL = ArrayLabels
module ListL = ListLabels

let dbg = Printf.printf "[debug]%s"

let max_num = 100_000_000_000

let id = fun x -> x
let tuple2 x y = (x,y)
let tuple3 x y z = (x,y,z)
let tuple4 x y z w = (x,y,z,w)
let succ x = x + 1
let pred x = x - 1

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let scan fmt f = Scanf.sscanf (read_line ()) fmt f

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


let () =
  let rec aux () =
    let n = scan "%d" id in
    if n = 0 then  ()
    else
      let ls = Array.of_list @@ scan_lines n "%f %f %f %f" tuple4 in
      let uf = UnionFind.empty n in
      ListL.map (0 ++^ n) ~f:(fun i ->
          ListL.map (0 ++^ n) ~f:(fun j ->
              let (x0, y0, z0, d0) = ls.(i)  in
              let (x1,y1,z1,d1) = ls.(j) in
              let d =
                sqrt ((x0-.x1) *.(x0 -.x1)+. (y0-.y1) *.(y0 -. y1) +. (z0 -. z1)*.(z0 -. z1)) in
              let diff = max 0.0000001 (d -. (d0+.d1)) in
              (diff, i, j)))
      |> List.concat
      |> List.sort (fun (d0,_,_) (d1,_,_) -> compare d0 d1)
      |> ListL.fold_left ~init:0.0 ~f:(fun acc (d, i, j)->
          acc +. if UnionFind.same uf i j then 0.0
          else
            (UnionFind.unite uf i j;d))
      |> Printf.printf "%.03f\n"
      |> aux
  in
  aux ()
