module Array = struct
  include ArrayLabels

  let min arr = Array.fold_left (fun a v -> min a v) Int.max_int arr

  let sum arr = Array.fold_left (+) 0 arr

  let scanl arr ~init ~f =
    let acc = Array.make (1 + Array.length arr) init in
    ArrayLabels.iteri arr ~f:(fun i a -> acc.(i+1) <- f acc.(i) a);
    acc
end

module List = struct
  include ListLabels

  module Op = struct
    let (++) n m =
      let rec aux i acc =
        if i = m then List.rev @@ m::acc
        else aux (i+1) @@ i::acc in
      if n > m then [] else aux n []

    let (++^) n m = n ++ (m-1)
    let (--) n m = List.rev (m ++ n)
    let (--^) n m = (n -- (m+1))
  end

  open Op

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

  let scanl ls ~init ~f =
    let rec aux ls prev acc =
      match ls with
      | [] -> acc
      | v::ls ->
         let sum = f prev v in
         aux ls sum (sum::acc)
    in List.rev @@ aux ls init [init]

  let sum  = ListLabels.fold_left ~f:(+) ~init:0
  let min ~cmp ls = List.hd @@ ListLabels.stable_sort ~cmp ls
  let max ~cmp ls = List.hd @@ ListLabels.stable_sort ~cmp:(fun a b -> - (cmp a b)) ls
  let of_string s = ListLabels.map ~f:(String.get s) (0 ++^ String.length s)
end

open List.Op

module Scan = struct
  let scan fmt f = Scanf.sscanf (read_line ()) fmt f

  let lines n fmt f =
    List.map ~f:(fun _ -> scan fmt f) (0++^n)

  let list sep f =
    let line = read_line () in
    Str.split (Str.regexp_string sep) line
    |> List.map ~f

  let matrix n m e f =
    let arr = Array.make_matrix ~dimx:n ~dimy:m e in
    Array.iteri ~f:(fun i line ->
        let s = Scanf.Scanning.from_string @@ read_line () in
        Array.iteri ~f:(fun j _ ->
            arr.(i).(j) <- Scanf.bscanf s " %s" f;
          ) line) arr; arr
end

module String = struct
  include StringLabels

  let explode s =
    Array.init (String.length s) ~f:(fun i -> String.get s i)
end

module Util = struct
  let max_num = 100_000_000_000
  let min_num = -100_000_000_000
  let id = fun x -> x
  let tuple2 x y = (x,y)
  let tuple3 x y z = (x,y,z)
  let tuple4 x y z w = (x,y,z,w)
  let succ x = x + 1
  let pred x = x - 1
  let diff a b = if a > b then a - b else b - a
  let neg a = - a
  let between n x m = n <= x && x < m
  let sqrt_int n = int_of_float @@ sqrt @@ float_of_int n
end

module Debug = struct
  let debug = ref true

  let p msg = if !debug then Printf.eprintf "[debug] %s\n" msg else ()

  let li a = if !debug then
               List.map a ~f:string_of_int
               |> String.concat ~sep:";"
               |> Printf.eprintf "[debug] [%s]\n"
end

open Util

module UnionFind = struct
  type elem = {root: int; size: int}

  let make n = Array.init n ~f:(fun i -> {root=i; size=1})

  let root t n =
    let rec aux c =
      if c = t.(c).root then c
      else
        let p = aux t.(c).root in
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
  Debug.debug := true;
  let (n, m) = Scan.scan "%d %d" tuple2 in
  let ab = Scan.lines m "%d %d" tuple2 in
  let tbl = Hashtbl.create n in
  let uf = UnionFind.make @@ succ n in
  List.iter ab ~f:(fun (a, b) -> Hashtbl.add tbl a b);
  List.scanl (n--^1) ~init:0 ~f:(fun num a ->
      let rec aux num = function
        | [] -> num + 1
        | b::ls ->
           if UnionFind.same uf a b then aux num ls
           else
             (UnionFind.unite uf a b; aux (pred num) ls)
      in
      aux num @@ Hashtbl.find_all tbl a
    )
  |> List.rev
  |> List.iter ~f:(fun v -> Printf.printf "%d\n" v)
