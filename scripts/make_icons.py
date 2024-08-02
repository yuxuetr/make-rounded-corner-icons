from PIL import Image, ImageDraw


def add_rounded_corners(image_path, output_path, radius):
    # 打开图像
    img = Image.open(image_path).convert("RGBA")

    # 创建一个全白的掩码图像
    mask = Image.new("L", img.size, 0)
    draw = ImageDraw.Draw(mask)

    # 绘制四个圆角
    width, height = img.size
    draw.rectangle([radius, 0, width - radius, height], fill=255)
    draw.rectangle([0, radius, width, height - radius], fill=255)
    draw.pieslice([0, 0, 2 * radius, 2 * radius], 180, 270, fill=255)
    draw.pieslice([width - 2 * radius, 0, width, 2 * radius], 270, 360, fill=255)
    draw.pieslice([0, height - 2 * radius, 2 * radius, height], 90, 180, fill=255)
    draw.pieslice(
        [width - 2 * radius, height - 2 * radius, width, height], 0, 90, fill=255
    )

    # 应用掩码
    img.putalpha(mask)

    # 保存图像
    img.save(output_path)


# 使用示例
add_rounded_corners("assets/1280.png", "assets/output.png", 72)
