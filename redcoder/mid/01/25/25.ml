let dbg = Printf.printf "[debug]%s"

let scanf fmt f =
  let l = read_line () in
  Scanf.sscanf l fmt f

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr;
  arr

let between n x m = n <= x && x < m

let rec dfs h w mat groups g (i,j) =
  List.iter (fun (dx,dy) ->
      let (x, y) = i + dx, j + dy in
      if between 0 x h && between 0 y w
         && groups.(x).(y) = -1 && mat.(x).(y) = 1 then
        (groups.(x).(y) <- g;
         dfs h w mat groups g (x, y))
    ) [(-1,-1);(-1,0);(-1,1);(0,-1);(0,1);(1,-1);(1,0);(1,1)]

let () =
  let rec aux ()=
    let (w, h) = scanf "%d %d" @@ (fun x y -> (x, y)) in
    if h = 0 && w = 0 then ()
    else
      let mat = scan_matrix h w 0 int_of_string in
      let groups = Array.make_matrix h w (-1) in
      let g = ref 1 in
      Array.iteri (fun i line ->
          Array.iteri (fun j v ->
              if v = 1 && groups.(i).(j) = -1 then
                (groups.(i).(j) <- !g;
                 dfs h w mat groups !g (i, j);
                 g := !g + 1) ) line) mat;
      Array.fold_left (fun v line ->
          Array.fold_left max v line)
        0 groups
      |> Printf.printf "%d\n";
      aux ()
  in
  aux ()
