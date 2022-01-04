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

  let range ?(step=1) ?(i=`Inclusive) n m =
    let op = match i with
      | `Inclusive -> (>)
      | `Exclusive -> (>=)
    in
    let rec aux i acc =
      if op i m then List.rev @@ acc
      else aux (i+step) @@ i::acc in
    if n > m then [] else aux n []

  module Op = struct
    let (++) n m = range n  m
    let (++^) n m = range ~i:`Exclusive n m
    let (--) n m = List.rev (m ++ n)
    let (--^) n m = (n -- succ m)
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

  let get v c = Hashtbl.find_opt c v |> Option.value ~default:0

  let add v c =
    (match Hashtbl.find_opt c v with
     | None -> Hashtbl.add c v 1
     | Some n -> Hashtbl.replace c v (succ n));
    c

  let of_array arr =
    let c = make @@ Array.length arr in
    Array.fold_left arr ~init:c ~f:(fun c v -> add v c)

  let of_list ls =
    let c = make @@ List.length ls in
    List.fold_left ls ~init:c ~f:(fun c v -> add v c)
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

  let l_s ls = if !debug then
                 String.concat ~sep:"; " ls
                 |> Printf.eprintf "[debug] [%s]\n"

  let l ls ~f = List.map ls ~f:f |> l_s

  let li ls = l ls ~f:string_of_int
end

open Util

let () =
  let n = Scan.scan "%d" Fun.id in
  let lines = Scan.lines n "%d %d %d %d" tuple4 in
  let mat = Array.make_matrix ~dimx:1001 ~dimy:1001 0 in
  List.iter lines ~f:(fun (lx, ly, rx, ry) ->
      mat.(lx).(ly) <- succ mat.(lx).(ly);
      mat.(lx).(ry) <- pred mat.(lx).(ry);
      mat.(rx).(ly) <- pred mat.(rx).(ly);
      mat.(rx).(ry) <- succ mat.(rx).(ry);
    );
  List.iter (0++^1001) ~f:(fun i ->
      List.iter (1++^1001) ~f:(fun j ->
          mat.(i).(j) <- mat.(i).(j) + mat.(i).(j-1)
        )
    );
  List.iter (0++^1001) ~f:(fun j ->
      List.iter (1++^1001) ~f:(fun i ->
          mat.(i).(j) <- mat.(i).(j) + mat.(i-1).(j)
        )
    );
  let cnt = Counter.make n in
  List.iter (0++^1001) ~f:(fun i ->
      List.iter (0++^1001) ~f:(fun j ->
          Counter.add mat.(i).(j) cnt |> ignore
        )
    );
  List.iter (1++n) ~f:(fun i -> Printf.printf "%d\n" @@ Counter.get i cnt)
