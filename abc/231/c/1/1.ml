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

open List.Op

module Scan = struct
  let scan fmt f = Scanf.sscanf (read_line ()) fmt f

  let list ~sep cnv =
    let line = read_line () in
    Str.split (Str.regexp_string sep) line
    |> List.map ~f:cnv

  let lines n fmt f =
    List.map ~f:(fun _ -> scan fmt f) (0++^n)

  let matrix n m e conv =
    let arr = Array.make_matrix ~dimx:n ~dimy:m e in
    Array.iteri ~f:(fun i line ->
        let s = Scanf.Scanning.from_string @@ read_line () in
        Array.iteri ~f:(fun j _ ->
            arr.(i).(j) <- Scanf.bscanf s " %s" conv;
          ) line) arr; arr
end

module Util = struct
  let max_num = 100_000_000_000
  let id = fun x -> x
  let tuple2 x y = (x,y)
  let tuple3 x y z = (x,y,z)
  let succ x = x + 1
  let pred x = x - 1
  let diff a b = if a > b then a - b else b - a
  let neg a = - a
  let between n x m = n <= x && x < m
  let dbg = Printf.printf "[debug]%s"
end

open Util

let binary_search arr ~cmp =
  let rec aux l h =
    let m = (l + h) / 2 in
    if l + 1 = h then h
    else if cmp arr.(m) then
      aux m h
    else
      aux l m
  in
  aux (-1) @@ Array.length arr

let () =
  let (n, q) = Scan.scan "%d %d" tuple2 in
  let arr = Scan.list ~sep:" " int_of_string |> Array.of_list in
  Array.sort ~cmp:compare arr;
  List.iter (0 ++^ q) ~f:(fun _ ->
      let x = Scan.scan "%d" id in
      let i = binary_search arr ~cmp:(fun v -> v < x) in
      Printf.printf "%d\n" @@ n - i)
