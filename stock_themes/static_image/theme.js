function renderFrame() {
    return "/9j/4AAQSkZJRgABAQEAYABgAAD/2wBDAAIBAQIBAQICAgICAgICAwUDAwMDAwYEBAMFBwYHBwcGBwcICQsJCAgKCAcHCg0KCgsMDAwMBwkODw0MDgsMDAz/2wBDAQICAgMDAwYDAwYMCAcIDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAz/wAARCAHgAeADASIAAhEBAxEB/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwD9/KKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAorhv2nPjnZ/sx/s4+PPiNqFpNqFl4E8P32vTWkLbJLpbaB5jErEEAts2gkYGc1zH7Af7WK/t0fsbfD74uL4fm8Kjx3pS6l/ZMt4t41ll2Qr5yqgcfLkNtUkEZVTkAj7yk19m1/8At7mt9/K/u16B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KKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKAP/9k=";
}

module.exports = {renderFrame};