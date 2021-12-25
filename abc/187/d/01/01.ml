module Array = struct
  include ArrayLabels
  let min arr = Array.fold_left (fun a v -> min a v) Int.max_int arr
  let sum arr = Array.fold_left (+) 0 arr
  let last a = Array.get a @@ Array.length a - 1
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

module Option = struct
  type 'a t = 'a option
  let is_some = function
    | Some _ -> true
    | None -> false

  let get = function
    | Some v -> v
    | None -> raise @@ Invalid_argument "None"
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

let scan_array ~sep cnv = Array.of_list @@ scan_list ~sep cnv

let scan_lines n fmt f =
  List.map ~f:(fun _ -> scan fmt f) (0++^n)

let scan_matrix n m e conv =
  let arr = Array.make_matrix ~dimx:n ~dimy:m e in
  Array.iteri ~f:(fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri ~f:(fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr; arr

let n = scan "%d" id
type line = {
  total: int;
  a: int;
}
let ls = scan_lines n "%d %d" (fun a b -> {total=a+b; a=a})

let ls = List.sort ~cmp:(fun {total=tx; a=ax} {total=ty; a=ay} ->
    - Int.compare (tx + ax) (ty + ay)) ls

let t = Array.make n 0
let a = Array.make n 0

let rec binary_search f l r =
  if r - l  > 1 then
    let mid = (l + r) / 2 in
    if f mid then
      binary_search f l mid
    else binary_search f mid r
  else r

let () =
  List.iteri ~f:(fun i {total=tt; a=_} ->
      if i = 0 then Array.set t i tt
      else Array.set t i (tt + Array.get t (i-1))
    ) ls;
  List.iteri ~f:(fun i {total=_; a=b} ->
      if i = 0 then Array.set a i b
      else Array.set a i (b + Array.get a (i-1))
    ) ls;
  let at = Array.last a in
  let f v =
    let tv = if v = 0 then 0 else Array.get t (v - 1) in
    let av = if v = 0 then 0 else Array.get a (v - 1) in
    (* Printf.printf "%d %d %d\n" v tv (at - av); *)
    tv > at - av  in
  Printf.printf "%d\n" @@ binary_search f 0 n
