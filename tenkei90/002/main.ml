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
       let pws = powerset tl in
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

module String = struct
  include StringLabels

  let explode s =
    Array.init (String.length s) ~f:(fun i -> String.get s i)
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

let () =
  let n = Scan.scan "%d" Fun.id in
  let rec aux l m =
    let len = List.length l in
    if len = n && m = 0 then Printf.printf "%s\n" @@ String.concat ~sep:"" @@ List.rev l
    else if len > n then ()
    else if m < 0 then ()
    else
      (aux ("(" :: l) (succ m);
       aux (")" :: l) (pred m)) in
  aux [] 0
