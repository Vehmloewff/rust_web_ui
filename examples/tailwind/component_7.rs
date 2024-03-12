use rust_web_ui::*;

pub struct Example7;

pub struct Example7Props {}

impl Default for Example7Props {
	fn default() -> Example7Props {
		Example7Props { }
	}
}

impl Widget<'_> for Example7 {
	type Props = Example7Props;

	fn render(mut ctx: Ctx<'_>, props: Example7Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Color(Color::Fg(11)), Style::Padding(Size::Exact(32))]);
			props.set_attribute("style", "min-height: 360px");
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Width(Size::Exact(256)), Style::Noop("text-right")]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.set_attribute("x-data", "Components.menu({ open: true })");
					props.set_attribute("x-init", "init()");
					props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
					props.set_attribute("@click.away", "onClickAway($event)");
					props.styles(&[Style::Noop("relative"), Style::InlineBlock, Style::Noop("text-left")]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.child("1", Dynamic).run("button", |mut props| {
							props.set_attribute("type", "button");
							props.styles(&[Style::InlineFlex, Style::Width(Size::Full), Style::JustifyCenter, Style::Noop("gap-x-1.5"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
							props.set_attribute("id", "menu-button");
							props.set_attribute("x-ref", "button");
							props.set_attribute("@click", "onButtonClick()");
							props.set_attribute("@keyup.space.prevent", "onButtonEnter()");
							props.set_attribute("@keydown.enter.prevent", "onButtonEnter()");
							props.set_attribute("aria-expanded", "true");
							props.set_attribute("aria-haspopup", "true");
							props.set_attribute("x-bind:aria-expanded", "open.toString()");
							props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
							props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
							
							props.child("0", Label).run(|props| props.text("Options"));
							props.child("1", Icon).run(|mut props| {
								props.style(&[Style::Noop("-mr-1"), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::TextColor(Color::Fg(44))]);
							});
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.set_attribute("x-show", "open");
						props.set_attribute("x-transition:enter", "transition ease-out duration-100");
						props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
						props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
						props.set_attribute("x-transition:leave", "transition ease-in duration-75");
						props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
						props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
						props.styles(&[Style::Noop("absolute"), Style::Noop("right-0"), Style::Noop("z-10"), Style::MarginTop(Size::Exact(8)), Style::Width(Size::Exact(224)), Style::Noop("origin-top-right"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::Noop("shadow-lg"), Style::Noop("ring-1"), Style::Noop("ring-black"), Style::Noop("ring-opacity-5"), Style::OnFocus(&[Style::Noop("outline-none")])]);
						props.set_attribute("x-ref", "menu-items");
						props.set_attribute("x-description", "Dropdown menu, show/hide based on menu state.");
						props.set_attribute("x-bind:aria-activedescendant", "activeDescendant");
						props.set_attribute("role", "menu");
						props.set_attribute("aria-orientation", "vertical");
						props.set_attribute("aria-labelledby", "menu-button");
						props.set_attribute("tabindex", "-1");
						props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
						props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
						props.set_attribute("@keydown.tab", "open = false");
						props.set_attribute("@keydown.enter.prevent", "open = false; focusButton()");
						props.set_attribute("@keyup.space.prevent", "open = false; focusButton()");
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::PaddingY(Size::Exact(4))]);
							props.set_attribute("role", "none");
							
							props.child("1", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
								props.set_attribute("x-state:on", "Active");
								props.set_attribute("x-state:off", "Not Active");
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 0, 'text-gray-700': !(activeIndex === 0) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-0");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 0)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.text("Account settings"));
							});
							props.child("3", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 1, 'text-gray-700': !(activeIndex === 1) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-1");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 1)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.text("Support"));
							});
							props.child("5", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 2, 'text-gray-700': !(activeIndex === 2) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-2");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 2)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.text("License"));
							});
							props.child("7", Dynamic).run("form", |mut props| {
								props.set_attribute("method", "POST");
								props.set_attribute("action", "#");
								props.set_attribute("role", "none");
								
								props.child("1", Dynamic).run("button", |mut props| {
									props.set_attribute("type", "submit");
									props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::Width(Size::Full), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-left"), Style::Noop("text-sm")]);
									props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 3, 'text-gray-700': !(activeIndex === 3) }");
									props.set_attribute("role", "menuitem");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("id", "menu-item-3");
									props.set_attribute("@mouseenter", "onMouseEnter($event)");
									props.set_attribute("@mousemove", "onMouseMove($event, 3)");
									props.set_attribute("@mouseleave", "onMouseLeave($event)");
									props.set_attribute("@click", "open = false; focusButton()");
									
									props.child("0", Label).run(|props| props.text("Sign out"));
								});
							});
						});
					});
				});
			});
		});
	}
}
