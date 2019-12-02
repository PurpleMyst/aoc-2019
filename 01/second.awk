{
    while ($0 > 0) {
        $0=int($0 / 3) - 2
        if ($0 > 0) {
            num+=$0
        }
    }
}
END { print num }
