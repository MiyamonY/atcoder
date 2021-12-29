module Array = struct
  include ArrayLabels

  let min arr = Array.fold_left (fun a v -> min a v) Int.max_int arr

  let sum arr = Array.fold_left (+) 0 arr

  let findi arr ~f =
    let n = Array.length arr in
    let rec aux i =
      if i = n then None
      else if f i arr.(i) then Some i
      else aux @@ succ i
    in aux 0

  let scanl arr ~init ~f =
    let acc = Array.make (succ @@ Array.length arr) init in
    ArrayLabels.iteri arr ~f:(fun i a -> acc.(i+1) <- f acc.(i) a);
    acc

  let iscanl arr ~init ~f =
    ArrayLabels.init (Array.length arr) ~f:(fun i ->
        if i = 0 then f init arr.(i)
        else f arr.(i-1) arr.(i))
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
    let (--^) n m = ((m+1) -- n)
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

  let scanl l ~init ~f =
    let rec aux p acc = function
      | [] -> acc
      | v::l ->
         let w = f p v in
         aux w (w::acc) l
    in List.rev @@ aux init [init] l

  let iscanl l ~init ~f =
    let rec aux p acc = function
      | [] -> acc
      | v::l ->
         let w = f p v in
         aux v (w::acc) l
    in List.rev @@ aux init [] l

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

  let array sep f = list sep f |> Array.of_list

  let matrix n m e f =
    let arr = Array.make_matrix ~dimx:n ~dimy:m e in
    Array.iteri ~f:(fun i line ->
        let s = Scanf.Scanning.from_string @@ read_line () in
        Array.iteri ~f:(fun j _ ->
            arr.(i).(j) <- Scanf.bscanf s " %s" f;
          ) line) arr; arr
end

module Char = struct
  include Char

  let index c ~base = Char.code c - Char.code base

  let from_index code ~base =
    Char.chr @@ (code + Char.code base)
end

module String = struct
  include StringLabels

  let explode s =
    Array.init (String.length s) ~f:(fun i -> String.get s i)

  let join ~sep cs =
    StringLabels.concat ~sep:sep @@ List.map cs ~f:Char.escaped
end

module Counter = struct
  type 'a t = ('a, int) Hashtbl.t

  let make n = Hashtbl.create n

  let of_array arr =
    let c = make @@ Array.length arr in
    Array.iter arr ~f:(fun v ->
        match Hashtbl.find_opt c v with
        | None -> Hashtbl.add c v 1
        | Some n -> Hashtbl.replace c v (succ n));
    c

  let of_list l =
    let c = make @@ List.length l in
    List.iter l ~f:(fun v ->
        match Hashtbl.find_opt c v with
        | None -> Hashtbl.add c v 1
        | Some n -> Hashtbl.replace c v (succ n));
    c

  let get c v = Hashtbl.find_opt c v |> Option.value ~default:0
end

module Util = struct
  let max_num = 100_000_000_000
  let min_num = -100_000_000_000
  let tuple2 x y = (x,y)
  let tuple3 x y z = (x,y,z)
  let tuple4 x y z w = (x,y,z,w)
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
  let (h, w) = Scan.scan "%d %d" tuple2 in
  let q = Scan.scan "%d" Fun.id in
  let mat = Array.make_matrix ~dimx:h ~dimy:w false in
  let uf = UnionFind.make (h * w) in
  List.iter (0++^q) ~f:(fun _ ->
      let l = Scan.array " " int_of_string in
      if Array.length l = 3 then
        let r, c = pred l.(1), pred l.(2) in
        let i = r*w + c in
        (mat.(r).(c) <- true;
         if r - 1 >= 0 && mat.(r-1).(c) then UnionFind.unite uf i ((r-1)*w + c);
         if r + 1 < h && mat.(r+1).(c) then UnionFind.unite uf i ((r+1)*w + c);
         if c - 1 >= 0 && mat.(r).(c-1) then UnionFind.unite uf i (r*w + c - 1);
         if c + 1 < w && mat.(r).(c+1) then UnionFind.unite uf i (r*w + c + 1);
        )
      else
        let r0, c0, r1, c1 = pred l.(1), pred l.(2), pred l.(3), pred l.(4) in
        if mat.(r0).(c0) && mat.(r1).(c1) && UnionFind.same uf (r0 * w + c0) (r1 * w + c1)
        then
          Printf.printf "Yes\n"
        else
          Printf.printf "No\n")
