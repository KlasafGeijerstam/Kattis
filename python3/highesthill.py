input()
peaks = [int(x) for x in input().split()]

highest_peak = 0
peak = 1
while peak < len(peaks) - 1:
    
    height = peaks[peak]
    if peaks[peak - 1] > height or peaks[peak + 1] > height:
        peak += 1
        continue

    i = peak
    while i > 0 and peaks[i - 1] <= peaks[i]:
        i -= 1
    j = peak

    while j < len(peaks) - 1 and peaks[j + 1] <= peaks[j]:
        j += 1

    peak_height = min(height - peaks[i], height - peaks[j])
    highest_peak = max(highest_peak, peak_height)

    peak = j


print(highest_peak)