# Image Optimization Script for Soroban Cookbook
# This script optimizes images for web performance
# Requires: ImageMagick or similar image processing tools

param(
    [string]$ImageDir = "static/img"
)

Write-Host "Starting image optimization..." -ForegroundColor Green

# Function to optimize an image
function Optimize-Image {
    param(
        [string]$InputPath,
        [string]$OutputPath,
        [int]$MaxWidth = 600,
        [int]$Quality = 85
    )
    
    if (Test-Path $InputPath) {
        try {
            # Get original size
            $originalSize = (Get-Item $InputPath).Length
            Write-Host "Processing: $(Split-Path $InputPath -Leaf)"
            Write-Host "Original size: $([math]::Round($originalSize / 1KB, 2)) KB"
            
            # Try to convert using ImageMagick convert
            $convertCmd = "convert `"$InputPath`" -resize ${MaxWidth}x${MaxWidth} -quality $Quality `"$OutputPath`""
            Invoke-Expression $convertCmd
            
            if (Test-Path $OutputPath) {
                $newSize = (Get-Item $OutputPath).Length
                $reduction = [math]::Round((1 - $newSize / $originalSize) * 100, 1)
                Write-Host "Optimized size: $([math]::Round($newSize / 1KB, 2)) KB"
                Write-Host "Size reduction: $reduction%" -ForegroundColor Green
                Write-Host "---"
            }
        } catch {
            Write-Host "Failed to optimize $(Split-Path $InputPath -Leaf): $_" -ForegroundColor Red
        }
    }
}

# Main optimization process
$imgPath = Join-Path $PSScriptRoot ".." $ImageDir

if (Test-Path $imgPath) {
    Set-Location $imgPath
    
    # Optimize 404.png (the largest file)
    Optimize-Image -InputPath "404.png" -OutputPath "404.webp" -MaxWidth 600 -Quality 85
    
    # Optimize social card images
    Optimize-Image -InputPath "docusaurus-social-card.jpg" -OutputPath "docusaurus-social-card.webp" -MaxWidth 1200 -Quality 85
    Optimize-Image -InputPath "soroban-social-card.png" -OutputPath "soroban-social-card.webp" -MaxWidth 1200 -Quality 85
    
    Write-Host "Image optimization completed!" -ForegroundColor Green
} else {
    Write-Host "Image directory not found: $imgPath" -ForegroundColor Red
}
