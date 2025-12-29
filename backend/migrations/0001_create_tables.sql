-- 0001_create_tables.sql

-- projects
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    client_name TEXT,
    margin NUMERIC(5,2) NOT NULL DEFAULT 0.2,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- materials
CREATE TABLE materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    cost_per_unit NUMERIC(10,2) NOT NULL,
    pricing_unit TEXT NOT NULL CHECK (pricing_unit IN ('linear_ft','square_ft','unit')),
    waste_factor NUMERIC(4,2) NOT NULL DEFAULT 0.0,
    units_per_measure NUMERIC(6,4)
);

-- material_line_items
CREATE TABLE material_line_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    material_id UUID NOT NULL REFERENCES materials(id) ON DELETE RESTRICT,
    linear_ft NUMERIC(10,2),
    square_ft NUMERIC(10,2),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- labor
CREATE TABLE labor (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    hours NUMERIC(10,2) NOT NULL,
    rate_per_hour NUMERIC(10,2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
