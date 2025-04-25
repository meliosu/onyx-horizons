-- Populate clients
INSERT INTO client (name, inn, address, contact_person_email, contact_person_name, is_vip) VALUES
    ('City Administration', '1234567890', '100 Municipal Plaza', 'mayor@citytown.gov', 'Mayor Smith', TRUE),
    ('Sunshine Developments', '9876543210', '42 Business Park', 'j.wilson@sunshine.com', 'Jane Wilson', TRUE),
    ('MetroBuilders Inc.', '5678901234', '789 Corporate Drive', 'ceo@metrobuilders.com', 'Robert Johnson', FALSE),
    ('GreenSpace Foundation', '1239874560', '55 Nature Way', 'director@greenspace.org', 'Sarah Green', FALSE),
    ('PowerGrid Corp', '4567890123', '200 Energy Avenue', 'operations@powergrid.com', 'Michael Power', TRUE);

-- Populate employee (technical personnel)
INSERT INTO employee (class, first_name, last_name, middle_name, gender, photo, salary, phone_number) VALUES
    ('technical_personnel', 'Alex', 'Morgan', 'J', 'male', 'photos/alex.jpg', 75000, '+1-555-1001'),
    ('technical_personnel', 'Emily', 'Chen', NULL, 'female', 'photos/emily.jpg', 82000, '+1-555-1002'),
    ('technical_personnel', 'David', 'Wilson', 'R', 'male', 'photos/david.jpg', 78000, '+1-555-1003'),
    ('technical_personnel', 'Sophia', 'Martinez', NULL, 'female', 'photos/sophia.jpg', 90000, '+1-555-1004'),
    ('technical_personnel', 'James', 'Taylor', 'L', 'male', 'photos/james.jpg', 88000, '+1-555-1005'),
    ('technical_personnel', 'Olivia', 'Johnson', 'K', 'female', 'photos/olivia.jpg', 85000, '+1-555-1006'),
    ('technical_personnel', 'Nathan', 'Lee', NULL, 'male', 'photos/nathan.jpg', 79000, '+1-555-1007'),
    ('technical_personnel', 'Emma', 'Brown', 'S', 'female', 'photos/emma.jpg', 81000, '+1-555-1008'),
    ('technical_personnel', 'Ryan', 'Garcia', NULL, 'male', NULL, 76000, '+1-555-1009');

-- Populate technical personnel
INSERT INTO technical_personnel (id, qualification, position, education_level, software_skills, is_project_manager) VALUES
    (1, 'engineer', 'foreman', 'Master''s Degree', ARRAY['AutoCAD', 'Revit'], TRUE),
    (2, 'technologist', 'foreman', 'Bachelor''s Degree', ARRAY['BIM', 'Primavera'], FALSE),
    (3, 'technician', 'master', 'Associate''s Degree', ARRAY['AutoCAD'], FALSE),
    (4, 'engineer', NULL, 'PhD', ARRAY['AutoCAD', 'Revit', 'SketchUp'], TRUE),
    (5, 'technologist', 'master', 'Master''s Degree', ARRAY['Primavera', 'MS Project'], FALSE),
    (6, 'engineer', 'foreman', 'Bachelor''s Degree', ARRAY['AutoCAD', 'Civil 3D'], FALSE),
    (7, 'technician', NULL, 'Associate''s Degree', ARRAY['AutoCAD'], FALSE),
    (8, 'technologist', 'master', 'Bachelor''s Degree', ARRAY['BIM', 'Navisworks'], FALSE),
    (9, 'engineer', 'foreman', 'Master''s Degree', ARRAY['Revit', 'Tekla'], TRUE);

-- Populate subclasses of technical personnel
INSERT INTO engineer (id, pe_license_id) VALUES
    (1, 12345),
    (4, 23456),
    (6, 34567),
    (9, 45678);

INSERT INTO technologist (id, management_tools) VALUES
    (2, ARRAY['Agile', 'Lean', 'Kanban']),
    (5, ARRAY['Scrum', 'Six Sigma']),
    (8, ARRAY['PMBOK', 'Agile', 'Waterfall']);

INSERT INTO technician (id, safety_training_level) VALUES
    (3, 'Advanced'),
    (7, 'Intermediate');

-- Populate departments
INSERT INTO department (supervisor_id, name) VALUES
    (1, 'Residential Construction'),
    (4, 'Infrastructure Development'),
    (9, 'Energy Projects');

-- Populate areas
INSERT INTO area (department_id, supervisor_id, name) VALUES
    (1, 2, 'North Residential'),
    (1, 5, 'South Residential'),
    (2, 6, 'Roads and Highways'),
    (2, 8, 'Bridges and Tunnels'),
    (3, 3, 'Power Plants'),
    (3, 7, 'Renewable Energy');

-- Populate employee (worker)
INSERT INTO employee (class, first_name, last_name, middle_name, gender, photo, salary, phone_number) VALUES
    ('worker', 'John', 'Smith', 'A', 'male', 'photos/john.jpg', 55000, '+1-555-2001'),
    ('worker', 'Lisa', 'Brown', NULL, 'female', 'photos/lisa.jpg', 52000, '+1-555-2002'),
    ('worker', 'Mark', 'Davis', 'T', 'male', 'photos/mark.jpg', 54000, '+1-555-2003'),
    ('worker', 'Anna', 'Wilson', NULL, 'female', 'photos/anna.jpg', 53000, '+1-555-2004'),
    ('worker', 'Paul', 'Johnson', 'R', 'male', 'photos/paul.jpg', 56000, '+1-555-2005'),
    ('worker', 'Kevin', 'Miller', NULL, 'male', 'photos/kevin.jpg', 51000, '+1-555-2006'),
    ('worker', 'Maria', 'Garcia', 'L', 'female', 'photos/maria.jpg', 54000, '+1-555-2007'),
    ('worker', 'Thomas', 'White', 'B', 'male', 'photos/thomas.jpg', 53000, '+1-555-2008'),
    ('worker', 'Sarah', 'Moore', NULL, 'female', 'photos/sarah.jpg', 52000, '+1-555-2009'),
    ('worker', 'Steven', 'Taylor', 'D', 'male', 'photos/steven.jpg', 55000, '+1-555-2010'),
    ('worker', 'Jennifer', 'Anderson', 'M', 'female', 'photos/jennifer.jpg', 53000, '+1-555-2011'),
    ('worker', 'Daniel', 'Hall', NULL, 'male', 'photos/daniel.jpg', 54000, '+1-555-2012'),
    ('worker', 'Michael', 'Clark', 'J', 'male', NULL, 52000, '+1-555-2013'),
    ('worker', 'Jessica', 'Lewis', 'A', 'female', 'photos/jessica.jpg', 51000, '+1-555-2014'),
    ('worker', 'Robert', 'Young', NULL, 'male', 'photos/robert.jpg', 56000, '+1-555-2015');

-- Populate workers
INSERT INTO worker (id, profession, union_name) VALUES
    (10, 'electrician', 'Electrical Workers Union'),
    (11, 'plumber', 'Plumbers Association'),
    (12, 'welder', 'Welders United'),
    (13, 'driver', 'Heavy Vehicle Operators'),
    (14, 'mason', 'Brick Layers Union'),
    (15, 'electrician', 'Electrical Workers Union'),
    (16, 'plumber', NULL),
    (17, 'welder', 'Welders United'),
    (18, 'driver', 'Heavy Vehicle Operators'),
    (19, 'mason', 'Brick Layers Union'),
    (20, 'electrician', NULL),
    (21, 'plumber', 'Plumbers Association'),
    (22, 'welder', NULL),
    (23, 'driver', 'Heavy Vehicle Operators'),
    (24, 'mason', 'Historic Preservation Guild');

-- Populate subclasses of workers
INSERT INTO electrirican (id, voltage_specializaition) VALUES
    (10, 'High Voltage'),
    (15, 'Low Voltage'),
    (20, 'Commercial');

INSERT INTO plumber (id, pipe_specialization) VALUES
    (11, 'Residential'),
    (16, 'Commercial'),
    (21, 'Industrial');

INSERT INTO welder (id, welding_machine) VALUES
    (12, 'MIG Welder'),
    (17, 'TIG Welder'),
    (22, 'Arc Welder');

INSERT INTO driver (id, vehicle_type, number_of_accidents) VALUES
    (13, 'Dump Truck', 0),
    (18, 'Concrete Mixer', 1),
    (23, 'Crane', 0);

INSERT INTO mason (id, hq_restoration_skills) VALUES
    (14, FALSE),
    (19, FALSE),
    (24, TRUE);

-- Populate brigades
INSERT INTO brigade (brigadier_id) VALUES
    (10), -- Electrician brigade
    (11), -- Plumber brigade
    (12), -- Welder brigade
    (14), -- Mason brigade
    (13); -- Driver brigade

-- Populate brigade assignments
INSERT INTO assignment (brigade_id, worker_id) VALUES
    (1, 10), -- Brigadier in own brigade
    (1, 15),
    (1, 20),
    (2, 11), -- Brigadier in own brigade
    (2, 16),
    (2, 21),
    (3, 12), -- Brigadier in own brigade
    (3, 17),
    (3, 22),
    (4, 14), -- Brigadier in own brigade
    (4, 19),
    (4, 24),
    (5, 13), -- Brigadier in own brigade
    (5, 18),
    (5, 23);

-- Populate sites
INSERT INTO site (name, area_id, client_id, type, location, risk_level, description) VALUES
    ('Sunset Apartments', 1, 2, 'housing', POINT(40.7128, -74.0060), 'medium', 'Residential apartment complex with 200 units'),
    ('Downtown Bridge', 4, 1, 'bridge', POINT(40.7589, -73.9851), 'high', 'Steel arch bridge crossing the river'),
    ('City Solar Farm', 6, 5, 'power_plant', POINT(40.8075, -73.9626), 'medium', 'Solar panel farm generating 50MW'),
    ('Oakwood Park', 1, 4, 'park', POINT(40.7282, -73.9217), 'low', 'Urban park with recreational facilities'),
    ('Highway 95 Extension', 3, 1, 'road', POINT(40.6892, -74.0445), 'medium', 'Highway extension with 6 lanes'),
    ('North Coal Plant', 5, 5, 'power_plant', POINT(40.8258, -74.2090), 'high', 'Coal-fired power station'),
    ('Riverside Condos', 2, 3, 'housing', POINT(40.7031, -74.0160), 'medium', 'Luxury condominium development');

-- Populate site subtypes
INSERT INTO housing (site_id, number_of_floors, number_of_entrances, type, energy_efficiency) VALUES
    (1, 12, 4, 'apartment', 'medium'),
    (7, 25, 2, 'condominium', 'high');

INSERT INTO bridge (site_id, length, road_material, max_load) VALUES
    (2, 350.5, 'asphalt', 500.0);

INSERT INTO power_plant (site_id, energy_output, energy_source, is_grid_connected) VALUES
    (3, 50.0, 'solar', TRUE),
    (6, 250.0, 'coal', TRUE);

INSERT INTO park (site_id, area, has_playground, has_lighting) VALUES
    (4, 2.5, TRUE, TRUE);

INSERT INTO road (site_id, length, lanes, surface) VALUES
    (5, 15000.0, 6, 'asphalt');

-- Populate equipment
INSERT INTO equipment (name, amount, purchase_date, purchase_cost, fuel_type) VALUES
    ('Excavator', 5, '2021-03-15', 250000.00, 'diesel'),
    ('Crane', 3, '2020-08-20', 400000.00, 'diesel'),
    ('Cement Mixer', 8, '2021-05-10', 75000.00, 'diesel'),
    ('Bulldozer', 4, '2022-01-05', 320000.00, 'diesel'),
    ('Forklift', 6, '2021-11-30', 60000.00, 'electric'),
    ('Generator', 10, '2022-02-15', 40000.00, 'gasoline'),
    ('Scaffolding Set', 20, '2021-04-25', 15000.00, NULL),
    ('Welding Machine', 15, '2021-07-12', 8000.00, 'electric'),
    ('Power Drill Set', 30, '2022-03-01', 5000.00, 'electric');

-- Populate equipment allocation
INSERT INTO equipment_allocation (equipment_id, department_id, site_id, amount, period_start, period_end) VALUES
    (1, 1, 1, 2, '2022-06-01', '2023-01-15'),
    (1, 1, 7, 1, '2022-07-15', '2023-03-30'),
    (1, 2, 5, 2, '2022-05-10', '2023-02-28'),
    (2, 2, 2, 2, '2022-04-01', '2023-01-20'),
    (2, 3, 3, 1, '2022-08-15', '2023-05-30'),
    (3, 1, 1, 3, '2022-06-01', '2023-01-15'),
    (3, 2, 5, 2, '2022-05-10', '2023-02-28'),
    (3, 3, 6, 3, '2022-09-01', '2023-06-15'),
    (4, 2, 5, 2, '2022-05-10', '2023-02-28'),
    (4, 3, 3, 1, '2022-08-15', '2023-05-30'),
    (4, 3, 6, 1, '2022-09-01', '2023-06-15'),
    (5, 1, NULL, 4, '2022-01-01', '2023-12-31'), -- Allocated to department only
    (6, 2, NULL, 6, '2022-01-01', '2023-12-31'), -- Allocated to department only
    (7, 1, 1, 8, '2022-06-01', '2023-01-15'),
    (7, 2, 2, 6, '2022-04-01', '2023-01-20'),
    (7, 3, 6, 6, '2022-09-01', '2023-06-15'),
    (8, 3, NULL, 15, '2022-01-01', '2023-12-31'), -- Allocated to department only
    (9, 1, NULL, 10, '2022-01-01', '2023-12-31'), -- Allocated to department only
    (9, 2, NULL, 10, '2022-01-01', '2023-12-31'), -- Allocated to department only
    (9, 3, NULL, 10, '2022-01-01', '2023-12-31'); -- Allocated to department only

-- Populate materials
INSERT INTO material (name, cost, units) VALUES
    ('Concrete', 120.00, 'cubic meter'),
    ('Steel Rebar', 1800.00, 'ton'),
    ('Brick', 0.75, 'piece'),
    ('Lumber', 600.00, 'cubic meter'),
    ('Glass', 45.00, 'square meter'),
    ('Insulation', 15.00, 'square meter'),
    ('Copper Wire', 8.50, 'meter'),
    ('PVC Pipe', 12.00, 'meter'),
    ('Asphalt', 100.00, 'ton'),
    ('Sand', 50.00, 'ton'),
    ('Gravel', 60.00, 'ton'),
    ('Paint', 35.00, 'gallon'),
    ('Solar Panel', 250.00, 'piece');

-- Populate tasks
INSERT INTO task (site_id, brigade_id, period_start, expected_period_end, actual_period_end, name, description) VALUES
    (1, 4, '2022-06-01', '2022-08-15', '2022-08-20', 'Foundation Construction', 'Pour concrete foundation for the apartment complex'),
    (1, 3, '2022-08-25', '2022-10-30', '2022-10-25', 'Steel Framework', 'Install steel framework for the building structure'),
    (1, 2, '2022-11-05', '2023-01-10', NULL, 'Plumbing Installation', 'Install water and sewage systems throughout the building'),
    (1, 1, '2023-01-20', '2023-03-30', NULL, 'Electrical Wiring', 'Install electrical systems and wiring'),
    
    (2, 5, '2022-04-01', '2022-05-15', '2022-05-20', 'Site Preparation', 'Clear and prepare site for bridge construction'),
    (2, 4, '2022-05-25', '2022-07-30', '2022-08-10', 'Pillar Construction', 'Build concrete pillars for bridge supports'),
    (2, 3, '2022-08-15', '2022-10-30', '2022-11-15', 'Steel Beam Installation', 'Install steel beams for bridge span'),
    (2, 2, '2022-11-20', '2022-12-20', '2022-12-15', 'Utilities Installation', 'Install water and power lines along bridge'),
    (2, 5, '2022-12-25', '2023-01-20', NULL, 'Road Surface Construction', 'Apply asphalt surface to the bridge roadway'),
    
    (3, 3, '2022-08-15', '2022-09-30', '2022-10-05', 'Steel Frame Installation', 'Install frames for solar panel mounts'),
    (3, 1, '2022-10-10', '2022-12-15', '2022-12-10', 'Solar Panel Installation', 'Install and wire solar panels'),
    (3, 1, '2022-12-20', '2023-02-28', NULL, 'Grid Connection', 'Connect solar farm to the power grid'),
    
    (4, 5, '2022-06-01', '2022-07-15', '2022-07-10', 'Land Clearing', 'Clear land and prepare for park construction'),
    (4, 4, '2022-07-20', '2022-09-20', '2022-09-30', 'Walkway Construction', 'Build walkways and paths throughout the park'),
    (4, 1, '2022-10-05', '2022-11-15', '2022-11-20', 'Lighting Installation', 'Install park lighting systems'),
    (4, 4, '2022-11-25', '2023-01-30', NULL, 'Playground Construction', 'Build playground and recreational facilities'),
    
    (5, 5, '2022-05-10', '2022-07-15', '2022-07-25', 'Land Grading', 'Grade land for highway extension'),
    (5, 4, '2022-08-01', '2022-10-15', '2022-10-30', 'Base Layer Construction', 'Construct road base layers'),
    (5, 5, '2022-11-05', '2023-01-15', '2023-01-25', 'Asphalt Paving', 'Apply asphalt surface to the highway'),
    (5, 1, '2023-02-01', '2023-02-28', NULL, 'Road Lighting Installation', 'Install street lights along highway');

-- Populate expenditures
INSERT INTO expenditure (task_id, material_id, expected_amount, actuial_amount) VALUES
    -- Foundation Construction for Sunset Apartments
    (1, 1, 500.0, 520.0),  -- Concrete
    (1, 2, 20.0, 22.0),    -- Steel Rebar
    (1, 10, 100.0, 105.0), -- Sand
    (1, 11, 150.0, 145.0), -- Gravel
    
    -- Steel Framework for Sunset Apartments
    (2, 2, 100.0, 98.0),   -- Steel Rebar
    
    -- Plumbing Installation for Sunset Apartments
    (3, 8, 2000.0, NULL),  -- PVC Pipe
    
    -- Electrical Wiring for Sunset Apartments
    (4, 7, 5000.0, NULL),  -- Copper Wire
    
    -- Site Preparation for Downtown Bridge
    (5, 11, 200.0, 210.0), -- Gravel
    
    -- Pillar Construction for Downtown Bridge
    (6, 1, 800.0, 850.0),  -- Concrete
    (6, 2, 50.0, 55.0),    -- Steel Rebar
    
    -- Steel Beam Installation for Downtown Bridge
    (7, 2, 200.0, 215.0),  -- Steel Rebar
    
    -- Utilities Installation for Downtown Bridge
    (8, 7, 1000.0, 980.0), -- Copper Wire
    (8, 8, 800.0, 790.0),  -- PVC Pipe
    
    -- Road Surface Construction for Downtown Bridge
    (9, 9, 150.0, NULL),   -- Asphalt
    
    -- Steel Frame Installation for City Solar Farm
    (10, 2, 80.0, 82.0),   -- Steel Rebar
    
    -- Solar Panel Installation for City Solar Farm
    (11, 13, 1000.0, 1000.0), -- Solar Panel
    (11, 7, 3000.0, 3050.0),  -- Copper Wire
    
    -- Grid Connection for City Solar Farm
    (12, 7, 2000.0, NULL), -- Copper Wire
    
    -- Land Clearing for Oakwood Park
    (13, 11, 50.0, 48.0),  -- Gravel
    
    -- Walkway Construction for Oakwood Park
    (14, 1, 200.0, 210.0), -- Concrete
    (14, 9, 30.0, 32.0),   -- Asphalt
    
    -- Lighting Installation for Oakwood Park
    (15, 7, 1500.0, 1520.0), -- Copper Wire
    
    -- Playground Construction for Oakwood Park
    (16, 4, 50.0, NULL),   -- Lumber
    (16, 12, 30.0, NULL),  -- Paint
    
    -- Land Grading for Highway 95 Extension
    (17, 10, 500.0, 520.0), -- Sand
    (17, 11, 700.0, 680.0), -- Gravel
    
    -- Base Layer Construction for Highway 95 Extension
    (18, 1, 1000.0, 1050.0), -- Concrete
    (18, 11, 800.0, 820.0),  -- Gravel
    
    -- Asphalt Paving for Highway 95 Extension
    (19, 9, 2000.0, 2100.0), -- Asphalt
    
    -- Road Lighting Installation for Highway 95 Extension
    (20, 7, 3000.0, NULL);   -- Copper Wire
