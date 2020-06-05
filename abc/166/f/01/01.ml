open Batteries
open Tuple
module EnumL = Enum.Labels
module ListL = List.Labels
module ArrayL = Array.Labels

let dbg0 x = Printf.eprintf "[debug]%s\n" @@ dump x
let dbg1 x = dbg0 x; x

let id = identity

let (++) n m = List.range n `To m
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

let lower_bound n f =
  let rec aux l u =
    if u - l > 1 then
      let m = (l + u) / 2 in
      if f m then aux l m
      else aux m u
    else u in aux (-1) n

let between n x m = n <= x && x < m

let (n,a,b,c) = scan "%d %d %d %d" Tuple4.make
let coms = scan_lines n "%s" id

open Option.Infix
open Option.Monad

let rec solve1 a b c= function
  | [] -> Some []
  | com::rest ->
    match com with
    | "AB" ->
      if a = 0 && b = 0 then None
      else if a = 1 then
        solve1 (pred a) (succ b) c rest >>= fun ret ->
        return ("B" :: ret)
      else
        solve1 (succ a) (pred b) c rest >>= fun ret ->
        return ("A" :: ret)
    | "BC" ->
      if b = 0 && c = 0 then None
      else if b = 1 then
        solve1 a (pred b) (succ c) rest >>= fun ret ->
        return ("C" :: ret)
      else
        solve1 a (succ b) (pred c) rest >>= fun ret ->
        return ("B" :: ret)
    | "AC" ->
      if a = 0 && c = 0 then None
      else if c = 1 then
        solve1 (succ a) b (pred c) rest >>= fun ret ->
        return ("A" :: ret)
      else solve1 (pred a) b (succ c) rest >>= fun ret ->
        return ("C" :: ret)
    | _ ->  None

let rec solven a b c = function
  | [] -> Some []
  | com :: rest ->
    begin
      match com with
      | "AB" ->
        if a > b then
          solven (pred a) (succ b) c rest >>= fun ret ->
          return ("B"::ret)
        else
          solven (succ a) (pred b) c rest >>= fun ret ->
          return ("A"::ret)
      | "BC" ->
        if b > c then
          solven a (pred b) (succ c) rest >>= fun ret ->
          return ("C"::ret)
        else
          solven a (succ b) (pred c) rest >>= fun ret ->
          return ("B"::ret)
      | "AC" ->
        if a > c then
          solven (pred a) b (succ c) rest >>= fun ret ->
          return ("C"::ret)
        else
          solven (succ a) b (pred c) rest >>= fun ret ->
          return ("A"::ret)
      | _ -> None
    end

let rec solve2 a b c = function
  | [] -> Some []
  | com :: rest ->
    begin
      match com with
      | "AB" ->
        if a = 1 && b = 1 && List.at_opt rest 0 |> Option.is_some then
          let next = List.at rest 0 in
          match next with
          | "AB" -> solve2 (pred a) (succ b) c rest >>= fun ret ->
            return ("B"::ret)
          | "AC" -> solve2 (succ a) (pred b) c rest >>= fun ret ->
            return ("A"::ret)
          | "BC" -> solve2 (pred a) (succ b) c rest >>= fun ret ->
            return ("B"::ret)
          | _ -> None
        else if a = 0 then
          solve2 (succ a) (pred b) c rest >>= fun ret ->
          return ("A" :: ret)
        else
          solve2 (pred a) (succ b) c rest >>= fun ret ->
          return ("B" :: ret)
      | "BC" ->
        if b = 1 && c= 1 && List.at_opt rest 0 |> Option.is_some then
          let next = List.at rest 0 in
          match next with
          | "AB" -> solve2 a (succ b) (pred c) rest >>= fun ret ->
            return ("B" :: ret)
          | "BC" -> solve2 a (pred b) (succ c) rest >>= fun ret ->
            return ("C"::ret)
          | "AC" -> solve2 a (pred b) (succ c) rest >>= fun ret ->
            return ("C"::ret)
          | _ -> None
        else if b = 0 then
          solve2 a (succ b) (pred c) rest >>= fun ret ->
          return ("B" :: ret)
        else
          solve2 a (pred b) (succ c) rest >>= fun ret ->
          return ("C" :: ret)
      | "AC" ->
        if c = 1 && a = 1 && List.at_opt rest 0 |> Option.is_some then
          let next = List.at rest 0 in
          match next with
          | "AB" -> solve2 (succ a) b (pred c) rest >>= fun ret ->
            return ("A" :: ret)
          | "BC" -> solve2 (pred a) b (succ c) rest >>= fun ret ->
            return ("C"::ret)
          | "AC" -> solve2 (succ a) b (pred c) rest >>= fun ret ->
            return ("A"::ret)
          | _ -> None
        else if c = 0 then
          solve2 (pred a) b (succ c) rest >>= fun ret ->
          return ("C" :: ret)
        else
          solve2 (succ a) b (pred c) rest >>= fun ret ->
          return ("A" :: ret)
      | _ -> None
    end

let () =
  (match List.first coms with
   | "AB" when a = 0 && b = 0 -> None
   | "BC" when b = 0 && c = 0 -> None
   | "AC" when a = 0 && c = 0 -> None
   | _ ->
     if a + b + c = 1 then
       solve1 a b c coms
     else if a + b + c = 2 then
       solve2 a b c coms
     else
       solven a b c coms)
  |> Option.map_default (fun ans -> "Yes\n" ^ String.join "\n" ans) "No"
  |> Printf.printf "%s\n"
