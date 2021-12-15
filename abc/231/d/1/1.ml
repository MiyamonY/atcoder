module Array = struct
  include ArrayLabels
  let min arr = Array.fold_left (fun a v -> min a v) Int.max_int arr
  let sum arr = Array.fold_left (+) 0 arr
end

module List = struct
  include ListLabels

  module Op = struct
    let (++) n m =
      let rec aux i =
        if i = m then [m]
        else i :: aux (i+1) in
      if n > m then [] else aux n

    let (++^) n m = n ++ (m-1)
    let (--) n m = List.rev (m ++ n)
    let (--^) n m = ((m+1) -- n)
  end

  open Op

  let init n ~f = ListLabels.map ~f (0++^n)

  let rec drop n = function
    | [] -> []
    | ls when n = 0 -> ls
    | _::ls -> drop (pred n) ls

  let rec take n = function
    | [] -> []
    | x::ls ->
       if n = 0 then []
       else x :: take (pred n) ls

  let findi ~f ls =
    let rec aux i =  function
      | [] -> i
      | x::ls -> if f x then i else aux (i+1) ls
    in
    aux 0 ls

  let rec powerset = function
    | [] -> [[]]
    | hd::tl ->
       let pws =  powerset tl in
       pws @ ListLabels.map pws ~f:(fun pw -> hd::pw)

  let permutations l =
    let rec interleave x = function
      | [] -> [[x]]
      | (hd::tl) as l ->
         (x::l) :: (interleave x tl |> ListLabels.map ~f:(fun l -> hd::l)) in
    let rec aux = function
      | [] -> [[]]
      | a::rest ->
         aux rest |> ListLabels.map ~f:(interleave a) |> List.concat in
    aux l

  let rec zip a b =
    match a, b with
    | [], _ -> []
    | _, [] -> []
    | x::a, y::b -> (x,y) :: zip a b

  let sum  = ListLabels.fold_left ~f:(+) ~init:0
  let min ~cmp ls = List.hd @@ ListLabels.stable_sort ~cmp ls
  let max ~cmp ls = List.hd @@ ListLabels.stable_sort ~cmp:(fun a b -> - (cmp a b)) ls
  let of_string s = ListLabels.map ~f:(String.get s) (0 ++^ String.length s)
end

module Util = struct
  let id = fun x -> x
  let tuple2 x y = (x,y)
  let tuple3 x y z = (x,y,z)
  let succ x = x + 1
  let pred x = x - 1
  let diff a b = if a > b then a - b else b - a
  let neg a = - a
  let between n x m = n <= x && x < m
end

open Util
open List.Op

let dbg = Printf.printf "[debug]%s"

let max_num = 100_000_000_000

let scan fmt f = Scanf.sscanf (read_line ()) fmt f

let scan_list ~sep cnv =
  let line = read_line () in
  Str.split (Str.regexp_string sep) line
  |> List.map ~f:cnv

let scan_lines n fmt f =
  List.map ~f:(fun _ -> scan fmt f) (0++^n)

let scan_matrix n m e conv =
  let arr = Array.make_matrix ~dimx:n ~dimy:m e in
  Array.iteri ~f:(fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri ~f:(fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr; arr


module UnionFind = struct
  type elem = {root: int; size: int}

  let make n = Array.init n ~f:(fun i -> {root=i; size=1})

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

let () =
  let (n, m) = scan "%d %d" tuple2 in
  let lines = scan_lines m "%d %d" tuple2 in
  let uf = UnionFind.make (succ n) in
  try
    let counts = Array.init (succ n)  ~f:(fun _ -> 0) in
    List.iter lines ~f:(fun (a, b) ->
           counts.(a) <- succ counts.(a);
           counts.(b) <- succ counts.(b);
           if counts.(a) > 2 || counts.(b) > 2 then failwith "error");
    List.iter lines ~f:(fun (a, b) ->
        if UnionFind.root uf a = UnionFind.root uf b then failwith "error"
        else
          UnionFind.unite uf a b
      );
    Printf.printf "Yes\n"
  with
  | _ -> Printf.printf "No\n"
